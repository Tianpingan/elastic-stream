package com.automq.elasticstream.client.tools.longrunning;

import org.apache.log4j.Logger;
import java.nio.ByteBuffer;
import java.util.Collections;
import java.util.HashSet;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.atomic.AtomicLong;
import java.util.concurrent.locks.Condition;
import java.util.concurrent.locks.Lock;

import com.automq.elasticstream.client.DefaultRecordBatch;
import com.automq.elasticstream.client.api.AppendResult;
import com.automq.elasticstream.client.api.Stream;

public class Producer implements Runnable {
    private static Logger log = Logger.getLogger(Producer.class.getClass());
    private Stream stream;
    private long startSeq;
    private int minSize;
    private int maxSize;
    private long interval;
    private long nextSeq;
    private AtomicLong endOffset;
    private AtomicBoolean stopped;
    private Lock lock;
    private Condition condition;
    private long previousTimestamp;
    // 10s
    public static final long TIME_OUT = 1000 * 10;

    public Producer(Stream stream, long startSeq, int minSize, int maxSize, long interval,
            AtomicLong endOffset, AtomicBoolean stopped, Lock lock, Condition condition) {
        this.stream = stream;
        this.startSeq = startSeq;
        this.minSize = minSize;
        this.maxSize = maxSize;
        this.nextSeq = startSeq;
        this.interval = interval;
        this.endOffset = endOffset;
        this.stopped = stopped;
        this.lock = lock;
        this.condition = condition;
    }

    @Override
    public void run() {
        log.info("Producer thread started");
        HashSet<Long> confirmOffset = new HashSet<Long>();
        this.previousTimestamp = System.currentTimeMillis();
        while (true) {
            if (this.stopped.get()) {
                return;
            }
            ByteBuffer buffer = Utils.getRecord(this.nextSeq, this.minSize, this.maxSize);
            CompletableFuture<AppendResult> cf = this.stream
                    .append(new DefaultRecordBatch(1, 0, Collections.emptyMap(),
                            buffer));
            log.info("[" + stream.streamId() + "]Send a append request, seq: " + this.nextSeq);
            cf.whenComplete((result, e) -> {
                if (e == null) {
                    long baseOffset = result.baseOffset();
                    confirmOffset.add(baseOffset + 1);
                    log.info("[" + stream.streamId() + "]Append a record batch, seq: " +
                            this.nextSeq
                            + ", offset:["
                            + baseOffset + ", " + (baseOffset + 1) + ")");
                } else {
                    log.error(e);
                    this.stopped.set(true);
                    this.lock.lock();
                    try {
                        this.condition.signalAll();
                    } finally {
                        this.lock.unlock();
                    }
                }
            });
            updateEndOffset(confirmOffset);
            long currentTimestamp = System.currentTimeMillis();
            long elapsedTime = currentTimestamp - previousTimestamp;
            if (elapsedTime > TIME_OUT) {
                log.error("[" + stream.streamId() + "]Timeout");
                this.stopped.set(true);
                this.lock.lock();
                try {
                    this.condition.signalAll();
                } finally {
                    this.lock.unlock();
                }
            }
            try {
                Thread.sleep(this.interval);
            } catch (InterruptedException e1) {
                log.error(e1.toString());
            }
            this.nextSeq++;
        }
    }

    void updateEndOffset(HashSet<Long> confirmOffset) {
        long offset = this.endOffset.get();
        while (confirmOffset.contains(offset + 1)) {
            confirmOffset.remove(offset + 1);
            offset++;
        }
        if (offset > this.endOffset.get()) {
            this.previousTimestamp = System.currentTimeMillis();
            this.endOffset.set(offset);
        }
    }
}
