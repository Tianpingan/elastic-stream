package com.automq.elasticstream.client.tools.e2e;

import static org.junit.Assert.assertFalse;
import static org.junit.Assert.assertTrue;
import java.util.concurrent.ExecutionException;
import com.automq.elasticstream.client.api.Client;
import com.automq.elasticstream.client.api.CreateStreamOptions;
import com.automq.elasticstream.client.api.Stream;

public class TrimAndDeleteTest {
    public static void main(String[] args) throws InterruptedException, ExecutionException {
        E2EOption option = new E2EOption();
        Client client = Client.builder().endpoint(option.getEndPoint()).kvEndpoint(option.getKvEndPoint())
                .build();
        Stream stream0 = client.streamClient()
                .createAndOpenStream(CreateStreamOptions.newBuilder().epoch(0)
                        .replicaCount(option.getReplica()).build())
                .get();
        // 1. 获取已经trim掉的记录
        assertTrue(Utils.appendRecords(stream0, 0, option.getCount(), 1));
        assertTrue(Utils.fetchRecords(stream0, 0, option.getCount() / 2, 1));
        stream0.trim(option.getCount() / 2).get();
        assertFalse(Utils.fetchRecords(stream0, 0, option.getCount() / 2, 1));
        stream0.close().get();
        // 2. 发送数据到已删除 Stream
        Stream stream1 = client.streamClient()
                .createAndOpenStream(CreateStreamOptions.newBuilder().epoch(0)
                        .replicaCount(option.getReplica()).build())
                .get();
        assertTrue(Utils.appendRecords(stream1, 0, option.getCount(), 1));
        assertTrue(Utils.fetchRecords(stream1, 0, option.getCount(), 1));
        stream1.destroy().get();
        assertFalse(Utils.fetchRecords(stream1, 0, option.getCount(), 1));
        assertFalse(Utils.appendRecordsWithTimeout(stream1, option.getCount(),
                option.getCount(), 1));
        assertFalse(Utils.fetchRecords(stream1, option.getCount(), option.getCount(), 1));
        System.out.println("PASS");
    }
}
