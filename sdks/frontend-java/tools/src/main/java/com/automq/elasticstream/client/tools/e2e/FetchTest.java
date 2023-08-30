package com.automq.elasticstream.client.tools.e2e;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;

import java.nio.charset.StandardCharsets;
import java.util.concurrent.ExecutionException;

import com.automq.elasticstream.client.api.Client;
import com.automq.elasticstream.client.api.CreateStreamOptions;
import com.automq.elasticstream.client.api.FetchResult;
import com.automq.elasticstream.client.api.RecordBatchWithContext;
import com.automq.elasticstream.client.api.Stream;

public class FetchTest {
    public static void main(String[] args) throws InterruptedException, ExecutionException {
        E2EOption option = new E2EOption();
        Client client = Client.builder().endpoint(option.getEndPoint()).kvEndpoint(option.getKvEndPoint())
                .build();
        // 1. 获取和重复获取一批数据
        Stream stream0 = client.streamClient()
                .createAndOpenStream(CreateStreamOptions.newBuilder().epoch(0)
                        .replicaCount(option.getReplica()).build())
                .get();
        assertTrue(Utils.appendRecords(stream0, 0, option.getCount(), option.getBatchSize()));
        // 获取方式a：one by one
        assertTrue(Utils.fetchRecords(stream0, 0, option.getCount(), option.getBatchSize()));
        // 获取方式b：all in one
        FetchResult fetchResult = stream0.fetch(0, option.getCount() * option.getBatchSize(), Integer.MAX_VALUE)
                .get();
        int len = fetchResult.recordBatchList().size();
        assertEquals(len, option.getCount());
        for (int i = 0; i < len; i++) {
            RecordBatchWithContext recordBatch = fetchResult.recordBatchList().get(i);
            assertEquals(i * option.getBatchSize(), recordBatch.baseOffset());
            assertEquals(i * option.getBatchSize() + option.getBatchSize(), recordBatch.lastOffset());
            byte[] rawPayload = new byte[recordBatch.rawPayload().remaining()];
            recordBatch.rawPayload().get(rawPayload);
            String payloadStr = new String(rawPayload, StandardCharsets.UTF_8);
            assertTrue(String.format("hello world %03d", i).equals(payloadStr));
        }
        stream0.close().get();
        // 2. 准备 Stream 和一个大批数据 从批中间读取部分数据
        Stream stream1 = client.streamClient()
                .createAndOpenStream(CreateStreamOptions.newBuilder().epoch(0)
                        .replicaCount(1).build())
                .get();
        assertTrue(Utils.appendRecords(stream1, 0, 2, 1024));
        assertTrue(Utils.checkFetchResult(stream1, 0, 512, 0, 1));
        assertTrue(Utils.checkFetchResult(stream1, 512, 1024, 0, 1));
        assertTrue(Utils.checkFetchResult(stream1, 512, 1024 + 1, 0, 2));
        assertTrue(Utils.checkFetchResult(stream1, 1024 - 1, 1024 + 1, 0, 2));
        assertTrue(Utils.checkFetchResult(stream1, 1024, 1024 + 1, 1, 2));
        System.out.println("PASS");
    }
}
