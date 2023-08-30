package com.automq.elasticstream.client.tools.e2e;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;

import java.nio.ByteBuffer;
import java.nio.charset.StandardCharsets;
import java.util.Collections;
import java.util.concurrent.ExecutionException;

import com.automq.elasticstream.client.DefaultRecordBatch;
import com.automq.elasticstream.client.api.Client;
import com.automq.elasticstream.client.api.CreateStreamOptions;
import com.automq.elasticstream.client.api.FetchResult;
import com.automq.elasticstream.client.api.OpenStreamOptions;
import com.automq.elasticstream.client.api.RecordBatchWithContext;
import com.automq.elasticstream.client.api.Stream;

public class MetadataTest {
    private static boolean openStream(Client client, long stream_id) {
        try {
            Stream stream = client.streamClient().openStream(stream_id, OpenStreamOptions.newBuilder().build()).get();
            System.out.println("Open, streamid: " + stream.streamId());
            return true;
            // return stream.streamId() == stream_id;
        } catch (InterruptedException | ExecutionException e) {
            return false;
        }
    }

    private static long createStream(Client client) {
        try {
            Stream stream = client.streamClient()
                    .createAndOpenStream(CreateStreamOptions.newBuilder().replicaCount(3).build()).get();
            System.out.println("CreateAndOpen, streamid: " + stream.streamId());
            return stream.streamId();
        } catch (InterruptedException | ExecutionException e) {
            return -1;
        }
    }

    private static ByteBuffer getBuffer(long idx) {
        byte[] payload = String.format("hello world %03d", idx).getBytes(StandardCharsets.UTF_8);
        ByteBuffer buffer = ByteBuffer.wrap(payload);
        return buffer;
    }

    private static boolean checkBuffer(long idx, byte[] buffer) {
        String payloadStr = new String(buffer, StandardCharsets.UTF_8);
        return payloadStr.equals(String.format("hello world %03d", idx));
    }

    public static void main(String[] args) throws InterruptedException, ExecutionException {
        E2EOption option = new E2EOption();
        Client client = Client.builder().endpoint(option.getEndPoint()).kvEndpoint(option.getKvEndPoint()).build();

        for (int i = 0; i < option.getCount(); i++) {
            Stream stream = client.streamClient()
                    .createAndOpenStream(CreateStreamOptions.newBuilder().replicaCount(3).build()).get();
            assertEquals(stream.streamId(), i);
            stream.close();
        }
        for (long i = 0; i < option.getCount(); i++) {
            Stream stream = client.streamClient().openStream(i, OpenStreamOptions.newBuilder().build()).get();
            assertEquals(stream.streamId(), i);
            stream.append(new DefaultRecordBatch(10, 0, Collections.emptyMap(), getBuffer(i))).get();
            stream.close();
        }
        for (long i = 0; i < option.getCount(); i++) {
            Stream stream = client.streamClient().openStream(i, OpenStreamOptions.newBuilder().build()).get();
            assertEquals(stream.streamId(), i);
            FetchResult fetchResult = stream.fetch(0, 10, Integer.MAX_VALUE).get();
            assertEquals(1, fetchResult.recordBatchList().size());
            RecordBatchWithContext recordBatch = fetchResult.recordBatchList().get(0);
            assertEquals(0, recordBatch.baseOffset());
            assertEquals(10, recordBatch.lastOffset());

            byte[] rawPayload = new byte[recordBatch.rawPayload().remaining()];
            recordBatch.rawPayload().get(rawPayload);
            assertTrue(checkBuffer(i, rawPayload));
            fetchResult.free();
        }
        for (long i = 0; i < option.getCount(); i++) {
            Stream stream = client.streamClient().openStream(i, OpenStreamOptions.newBuilder().build()).get();
            assertEquals(stream.streamId(), i);
            stream.close();
        }
        System.out.println("PASS");
    }
}
