
package com.automq.elasticstream.client.examples;

import java.nio.ByteBuffer;
import java.util.Collections;
import java.util.concurrent.BlockingQueue;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.ExecutionException;
import java.util.concurrent.LinkedBlockingQueue;
import java.util.zip.CRC32;

import com.automq.elasticstream.client.DefaultRecordBatch;
import com.automq.elasticstream.client.api.AppendResult;
import com.automq.elasticstream.client.api.Client;
import com.automq.elasticstream.client.api.CreateStreamOptions;
import com.automq.elasticstream.client.api.FetchResult;
import com.automq.elasticstream.client.api.RecordBatchWithContext;
import com.automq.elasticstream.client.api.Stream;
import java.util.Random;

public class LongRunning {
    public static void main(String[] args) throws Exception {
        LongRunningOption option = new LongRunningOption();
        System.out.println(option.getEndPoint() + ", " + option.getKvEndPoint());
        Client client = Client.builder().endpoint(option.getEndPoint()).kvEndpoint(option.getKvEndPoint()).build();
        while (true) {
            Stream stream = client.streamClient()
                    .createAndOpenStream(
                            CreateStreamOptions.newBuilder().replicaCount(option.getReplicaCount()).build())
                    .get();
            long streamId = stream.streamId();
            System.out.println("Create StreamID: " + streamId);
            BlockingQueue<Elem> crcQueue = new LinkedBlockingQueue<>(1024);
            Thread producerThread = new Thread(new Producer(crcQueue, stream));
            Thread consumerThread = new Thread(new Consumer(crcQueue, producerThread, stream));
            producerThread.start();
            consumerThread.start();

            producerThread.join();
            consumerThread.join();
            stream.close().get();
        }
    }
}

class Elem {
    long crc;
    long offset;

    Elem(long crc, long offset) {
        this.crc = crc;
        this.offset = offset;
    }

    long getCrc() {
        return this.crc;
    }

    long getOffset() {
        return this.offset;
    }
}

class LongRunningOption {
    private String endpoint = "127.0.0.1:12378";
    private String kvEndpoint = "127.0.0.1:12379";
    private int replicaCount = 1;

    public LongRunningOption() {
        String endpoint = System.getenv("END_POINT");
        if (endpoint != null) {
            this.endpoint = endpoint;
        }
        String kvEndpoint = System.getenv("KV_END_POINT");
        if (kvEndpoint != null) {
            this.kvEndpoint = kvEndpoint;
        }
        String replicaCountStr = System.getenv("REPLICA_COUNT");
        if (replicaCountStr != null) {
            this.replicaCount = Integer.parseInt(replicaCountStr);
        }
    }

    public String getEndPoint() {
        return this.endpoint;
    }

    public String getKvEndPoint() {
        return this.kvEndpoint;
    }

    public int getReplicaCount() {
        return this.replicaCount;
    }
}

class Utils {
    public static byte[] generateRandomByteArray(int min, int max) {
        int length = getRandomLength(min, max);
        byte[] byteArray = new byte[length];
        new Random().nextBytes(byteArray);
        return byteArray;
    }

    public static int getRandomLength(int min, int max) {
        Random random = new Random();
        return min + random.nextInt(max - min);
    }

    public static long calculateCRC32(byte[] byteArray) {
        CRC32 crc32 = new CRC32();
        crc32.update(byteArray);
        return crc32.getValue();
    }
}

class Producer implements Runnable {
    private final BlockingQueue<Elem> crcQueue;
    private Stream stream;

    public Producer(BlockingQueue<Elem> crcQueue, Stream stream) {
        this.crcQueue = crcQueue;
        this.stream = stream;
    }

    @Override
    public void run() {
        while (true) {
            byte[] payload = Utils.generateRandomByteArray(1024, 4096);
            long crc32 = Utils.calculateCRC32(payload);
            ByteBuffer buffer = ByteBuffer.wrap(payload);
            CompletableFuture<AppendResult> cf = stream
                    .append(new DefaultRecordBatch(1024, 0, Collections.emptyMap(), buffer));
            cf.whenComplete((rst, ex) -> {
                if (ex == null) {
                    long offset = rst.baseOffset();
                    try {
                        crcQueue.put(new Elem(crc32, offset));
                    } catch (InterruptedException e) {
                        // TODO Auto-generated catch block
                        e.printStackTrace();
                    }
                    System.out.println(
                            "Append success: " + offset);
                    crcQueue.notifyAll();
                }
            });
        }

        // }
        // }
    }
}

class Consumer implements Runnable {
    private final BlockingQueue<Elem> crcQueue;
    private final Thread producerThread;
    private Stream stream;

    public Consumer(BlockingQueue<Elem> crcQueue, Thread producerThread, Stream stream) {
        this.crcQueue = crcQueue;
        this.producerThread = producerThread;
        this.stream = stream;
    }

    @Override
    public void run() {
        try {
            while (true) {
                Elem elem = crcQueue.take();
                FetchResult fetchResult = stream.fetch(elem.getOffset(), elem.getOffset() +
                        10, Integer.MAX_VALUE)
                        .get();
                RecordBatchWithContext recordBatch = fetchResult.recordBatchList().get(0);
                byte[] rawPayload = new byte[recordBatch.rawPayload().remaining()];
                recordBatch.rawPayload().get(rawPayload);
                System.out.println(
                        "base offset: " + recordBatch.baseOffset() + " last offset: " +
                                recordBatch.lastOffset());
                long crc0 = elem.getCrc();
                long crc = Utils.calculateCRC32(rawPayload);
                if (crc != crc0) {
                    System.out.println("Fetch Error!");
                    return;
                }
                System.out.println("Fetch success: " + elem.getOffset());
                fetchResult.free();

            }
        } catch (InterruptedException | ExecutionException e) {
            Thread.currentThread().interrupt();
        }
    }
}
