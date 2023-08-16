package com.automq.elasticstream.client.tools.e2e.tests;

import static org.junit.Assert.assertEquals;

import java.util.concurrent.ExecutionException;

import com.automq.elasticstream.client.api.Client;
import com.automq.elasticstream.client.api.CreateStreamOptions;
import com.automq.elasticstream.client.api.OpenStreamOptions;
import com.automq.elasticstream.client.api.Stream;
import com.automq.elasticstream.client.tools.e2e.E2EOption;

public class MetadataTest {
    private static boolean openStream(Client client, long stream_id) {
        try {
            Stream stream = client.streamClient().openStream(stream_id, OpenStreamOptions.newBuilder().build()).get();
            return stream.streamId() == stream_id;
        } catch (InterruptedException | ExecutionException e) {
            return false;
        }
    }

    private static long createStream(Client client) {
        try {
            Stream stream = client.streamClient()
                    .createAndOpenStream(CreateStreamOptions.newBuilder().replicaCount(3).build()).get();
            return stream.streamId();
        } catch (InterruptedException | ExecutionException e) {
            return -1;
        }
    }

    public static void main(String[] args) {
        E2EOption option = new E2EOption();
        Client client = Client.builder().endpoint(option.getEndPoint()).kvEndpoint(option.getKvEndPoint()).build();
        for (int i = 0; i < option.getCount(); i++) {
            assertEquals(i, createStream(client));
        }
        for (long i = 0; i < option.getCount(); i++) {
            assertEquals(true, openStream(client, i));
        }
        System.out.println("PASS");
    }
}
