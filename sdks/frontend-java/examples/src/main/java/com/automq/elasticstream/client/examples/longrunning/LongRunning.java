package com.automq.elasticstream.client.examples.longrunning;

import org.apache.log4j.Logger;

import java.nio.ByteBuffer;

import java.util.Collections;
import java.util.List;
import java.util.concurrent.ExecutionException;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.atomic.AtomicLong;
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
    private static Logger log = Logger.getLogger(LongRunning.class.getClass());

    public static void main(String[] args) throws Exception {
        LongRunningOption option = new LongRunningOption();
        log.info("EndPoint: " + option.getEndPoint() + ", KvEndPoint: " +
                option.getKvEndPoint()
                + ", AppendInterval: " +
                option.getInterval()
                + ", PayloadSizeMin: "
                + option.getMin() + ", PayloadSizeMax: " + option.getMax());
        Client client = Client.builder().endpoint(option.getEndPoint()).kvEndpoint(option.getKvEndPoint()).build();
        ExecutorService executor = Executors.newFixedThreadPool(9);
        for (int replica = 1; replica <= 3; replica++) {
            Stream stream = client.streamClient()
                    .createAndOpenStream(
                            CreateStreamOptions.newBuilder().replicaCount(replica).build())
                    .get();
            createTask(stream, Utils.getRandomInt(0, 1024), option.getMin(),
                    option.getMax(), option.getInterval(), executor);
        }
    }

    private static void createTask(Stream stream,
            int startSeq,
            int minSize,
            int maxSize,
            long interval, ExecutorService executor) {
        AtomicLong endOffset = new AtomicLong(0);
        executor.submit(new Producer(stream, startSeq, minSize, maxSize, interval, endOffset));
        executor.submit(new TailReadConsumerThread(stream,
                startSeq, endOffset));
        executor.submit(new RepeatedReadConsumerThread(stream, startSeq, endOffset));
    }
}

class Producer implements Runnable {
    private static Logger log = Logger.getLogger(Producer.class.getClass());
    private Stream stream;
    private long startSeq;
    private int minSize;
    private int maxSize;
    private long interval;
    private long nextSeq;
    private long lastSeq;
    private ByteBuffer lastRecord;
    private AtomicLong endOffset;

    public Producer(Stream stream, long startSeq, int minSize, int maxSize, long interval,
            AtomicLong endOffset) {
        this.stream = stream;
        this.startSeq = startSeq;
        this.minSize = minSize;
        this.maxSize = maxSize;
        this.nextSeq = startSeq;
        this.interval = interval;
        this.lastSeq = -1;
        this.endOffset = endOffset;
    }

    @Override
    public void run() {
        log.info("Producer thread started");
        while (true) {
            try {
                ByteBuffer buffer;
                if (this.lastSeq != this.nextSeq) {
                    buffer = Utils.getRecord(this.nextSeq, this.minSize, this.maxSize);
                    this.lastRecord = buffer;
                    this.lastSeq = this.nextSeq;
                } else {
                    log.info("Retry append...");
                    buffer = this.lastRecord;
                }
                AppendResult result = this.stream
                        .append(new DefaultRecordBatch(1, 0, Collections.emptyMap(), buffer)).get();
                log.info("Append a record batch, seq: " + this.nextSeq + ", offset: " + result.baseOffset());
                this.endOffset.set(result.baseOffset() + 1);
                this.nextSeq++;
            } catch (InterruptedException | ExecutionException e) {
                log.error(e.toString());
            }
        }
    }
}

class TailReadConsumerThread implements Runnable {

    private static Logger log = Logger.getLogger(TailReadConsumerThread.class.getClass());
    private Stream stream;
    private long startSeq;
    private long nextSeq;
    private long nextOffset;
    private AtomicLong endOffset;

    public TailReadConsumerThread(Stream stream, long startSeq, AtomicLong endOffset) {
        this.stream = stream;
        this.startSeq = startSeq;
        this.nextSeq = startSeq;
        this.nextOffset = 0;
        this.endOffset = endOffset;
    }

    @Override
    public void run() {
        log.info("TailReadConsumerThread thread started");
        while (true) {
            try {
                long endOffset = this.endOffset.get();
                log.info("Tail read, nextSeq: " + this.nextSeq + ", nextOffset: " + nextOffset + ", endOffset: "
                        + endOffset);
                FetchResult fetchResult = this.stream.fetch(this.nextOffset, endOffset,
                        Integer.MAX_VALUE).get();
                List<RecordBatchWithContext> recordBatch = fetchResult.recordBatchList();
                log.info("Tail read, fetch a recordBatch, size: " + recordBatch.size());
                for (int i = 0; i < recordBatch.size(); i++) {
                    RecordBatchWithContext record = recordBatch.get(i);
                    byte[] rawPayload = new byte[record.rawPayload().remaining()];
                    record.rawPayload().get(rawPayload);
                    if (Utils.checkRecord(this.nextSeq, ByteBuffer.wrap(rawPayload)) == false) {
                        log.error("Tail read, something wrong with record[" + i + "]");
                    } else {
                        this.nextSeq++;
                    }
                }
                this.nextOffset = this.nextOffset + recordBatch.size();
                fetchResult.free();
            } catch (InterruptedException | ExecutionException e) {
                log.error(e.toString());
            }
        }
    }
}

class RepeatedReadConsumerThread implements Runnable {

    private static Logger log = Logger.getLogger(RepeatedReadConsumerThread.class.getClass());
    private Stream stream;
    private long startSeq;
    private AtomicLong endOffset;

    public RepeatedReadConsumerThread(Stream stream, long startSeq, AtomicLong endOffset) {
        this.stream = stream;
        this.startSeq = startSeq;
        this.endOffset = endOffset;
    }

    @Override
    public void run() {
        log.info("RepeatedReadConsumer thread started");
        while (true) {
            try {
                long endOffset = this.endOffset.get();
                long startOffset = this.stream.startOffset();
                log.info("Repeated read, startOffset: " + startOffset + ", endOffset: "
                        + endOffset);
                FetchResult fetchResult = this.stream.fetch(startOffset, endOffset, Integer.MAX_VALUE).get();
                List<RecordBatchWithContext> recordBatch = fetchResult.recordBatchList();
                log.info("Repeated read, fetch a recordBatch, size: " + recordBatch.size());
                long nextSeq = this.startSeq;
                for (int i = 0; i < recordBatch.size(); i++) {
                    RecordBatchWithContext record = recordBatch.get(i);
                    byte[] rawPayload = new byte[record.rawPayload().remaining()];
                    record.rawPayload().get(rawPayload);
                    if (Utils.checkRecord(nextSeq, ByteBuffer.wrap(rawPayload)) == false) {
                        log.error("Repeated read, something wrong with record[" + i + "]");
                    } else {
                        nextSeq++;
                    }
                }
                fetchResult.free();
            } catch (InterruptedException | ExecutionException e) {
                log.error(e.toString());
                continue;
            }
        }
    }
}

class LongRunningOption {
    private String endpoint = "127.0.0.1:12378";
    private String kvEndpoint = "127.0.0.1:12379";
    private long appendInterval = 100;
    private int payloadSizeMin = 1024;
    private int payloadSizeMax = 4096;

    public LongRunningOption() {
        String endpoint = System.getenv("END_POINT");
        if (endpoint != null) {
            this.endpoint = endpoint;
        }
        String kvEndpoint = System.getenv("KV_END_POINT");
        if (kvEndpoint != null) {
            this.kvEndpoint = kvEndpoint;
        }
        String intervalStr = System.getenv("APPEND_INTERVAL");
        if (intervalStr != null) {
            this.appendInterval = Long.parseLong(intervalStr);
        }
        String minStr = System.getenv("PAYLOAD_SIZE_MIN");
        if (minStr != null) {
            this.payloadSizeMin = Integer.parseInt(minStr);
        }
        String maxStr = System.getenv("PAYLOAD_SIZE_MAX");
        if (maxStr != null) {
            this.payloadSizeMax = Integer.parseInt(maxStr);
        }
    }

    public String getEndPoint() {
        return this.endpoint;
    }

    public String getKvEndPoint() {
        return this.kvEndpoint;
    }

    public long getInterval() {
        return this.appendInterval;
    }

    public int getMin() {
        return this.payloadSizeMin;
    }

    public int getMax() {
        return this.payloadSizeMax;
    }
}

class Utils {

    private static Logger log = Logger.getLogger(Utils.class.getClass());

    public static byte[] generateRandomByteArray(int min, int max) {
        int length = getRandomInt(min, max);
        byte[] byteArray = new byte[length];
        new Random().nextBytes(byteArray);
        return byteArray;
    }

    public static int getRandomInt(int min, int max) {
        Random random = new Random();
        return min + random.nextInt(max - min);
    }

    public static long calculateCRC32(byte[] byteArray) {
        CRC32 crc32 = new CRC32();
        crc32.update(byteArray);
        return crc32.getValue();
    }

    public static ByteBuffer getRecord(long seq, int min, int max) {
        byte[] payload = generateRandomByteArray(min, max);
        long crc32 = calculateCRC32(payload);
        ByteBuffer buffer = ByteBuffer.allocate(Long.BYTES + Long.BYTES +
                payload.length);
        buffer.putLong(seq);
        buffer.putLong(crc32);
        buffer.put(payload);
        buffer.flip();
        return buffer;
    }

    public static Boolean checkRecord(long seq0, ByteBuffer buffer) {
        long seq = buffer.getLong();
        if (seq != seq0) {
            log.error("Out of order, expect seq: " + seq0 + ", but get: " + seq);
            return false;
        }
        long crc = buffer.getLong();
        byte[] payload = new byte[buffer.remaining()];
        buffer.get(payload);
        long crc0 = calculateCRC32(payload);
        if (crc0 != crc) {
            log.error("Payload corrupted, expect crc32: " + crc + ", but get: " + crc0);
            return false;
        }
        return true;
    }
}
