package com.automq.elasticstream.client.examples;

import static org.junit.Assert.assertEquals;

import java.nio.ByteBuffer;
import java.nio.charset.StandardCharsets;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.ExecutionException;
import java.nio.ByteBuffer;
import java.nio.charset.StandardCharsets;
import java.util.Collections;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.TimeUnit;

import com.automq.elasticstream.client.DefaultRecordBatch;
import com.automq.elasticstream.client.api.Client;
import com.automq.elasticstream.client.api.CreateStreamOptions;
import com.automq.elasticstream.client.api.FetchResult;
import com.automq.elasticstream.client.api.OpenStreamOptions;
import com.automq.elasticstream.client.api.RecordBatchWithContext;
import com.automq.elasticstream.client.api.Stream;
import com.automq.elasticstream.client.api.AppendResult;

// 创建stream，不断往里面写
class Writer implements Runnable {
    private Stream stream;

    public Writer(Stream stream) {
        this.stream = stream;
    }

    @Override
    public void run() {
        int counter = -1;
        while (true) {
            counter++;
            int index = counter;
            byte[] payload = String.format("hello world %03d", index).getBytes(StandardCharsets.UTF_8);
            ByteBuffer buffer = ByteBuffer.wrap(payload);
            long startNanos = System.nanoTime();
            CompletableFuture<AppendResult> cf = this.stream
                    .append(new DefaultRecordBatch(10, 0, Collections.emptyMap(), buffer));
            System.out.println("append " + index + " async cost:" + (System.nanoTime() - startNanos) / 1000 + "us");
            cf.whenComplete((rst, ex) -> {
                if (ex == null) {
                    long offset = rst.baseOffset();
                    assertEquals(index * 10, offset);
                    System.out.println(
                            "append " + index + " callback cost:" + (System.nanoTime() - startNanos) / 1000 + "us");
                }
            });
        }
    }

};

// 打开stream，不断从中读取
class Reader implements Runnable {
    private Stream stream;

    public Reader(Stream stream) throws InterruptedException, ExecutionException {
        this.stream = stream;
    }

    @Override
    public void run() {
        int counter = -1;
        while (true) {
            counter++;
            int index = counter;
            try {
                System.out.println("fetch[" + index * 10 + ", " + (index * 10 + 10) + "]");
                FetchResult fetchResult = stream.fetch(index * 10, index * 10 + 10, Integer.MAX_VALUE).get();
                assertEquals(1, fetchResult.recordBatchList().size());
                RecordBatchWithContext recordBatch = fetchResult.recordBatchList().get(0);
                assertEquals(index * 10, recordBatch.baseOffset());
                assertEquals(index * 10 + 10, recordBatch.lastOffset());

                byte[] rawPayload = new byte[recordBatch.rawPayload().remaining()];
                recordBatch.rawPayload().get(rawPayload);
                String payloadStr = new String(rawPayload, StandardCharsets.UTF_8);
                System.out.println("fetch record result offset[" + recordBatch.baseOffset() + ","
                        + recordBatch.lastOffset() + "]" + " payload:" + payloadStr + ".");
                assertEquals(String.format("hello world %03d", index), payloadStr);
                fetchResult.free();
            } catch (InterruptedException | ExecutionException e) {
                System.out.println("e = " + e.toString());
                counter--;
            }
        }
    }

}

public class LongRunning {

    public static void main(String[] args) throws InterruptedException, ExecutionException {
        Client client = Client.builder().endpoint("127.0.0.1:12378").kvEndpoint("127.0.0.1:12379").build();
        Stream stream = client.streamClient()
                .createAndOpenStream(CreateStreamOptions.newBuilder().replicaCount(1).build()).get();
        // 创建一个写线程并启动
        Thread writerThread = new Thread(new Writer(stream));
        writerThread.start();

        // 创建一个读线程并启动
        Thread readerThread = new Thread(new Reader(stream));
        readerThread.start();
    }
}
