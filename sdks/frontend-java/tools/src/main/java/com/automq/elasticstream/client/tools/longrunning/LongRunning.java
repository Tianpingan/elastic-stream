package com.automq.elasticstream.client.tools.longrunning;

import org.apache.log4j.Logger;

import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.atomic.AtomicLong;
import java.util.concurrent.locks.Condition;
import java.util.concurrent.locks.Lock;
import java.util.concurrent.locks.ReentrantLock;
import com.automq.elasticstream.client.api.Client;
import com.automq.elasticstream.client.api.CreateStreamOptions;
import com.automq.elasticstream.client.api.Stream;

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
        Client client = Client.builder().endpoint(option.getEndPoint()).kvEndpoint(option.getKvEndPoint())
                .build();
        ExecutorService executor = Executors.newFixedThreadPool(9);
        for (int replica = 1; replica <= 3; replica++) {
            int finalReplica = replica;
            executor.submit(() -> {
                while (true) {
                    Lock lock = new ReentrantLock();
                    Condition condition = lock.newCondition();
                    Stream stream = client.streamClient()
                            .createAndOpenStream(CreateStreamOptions.newBuilder().replicaCount(finalReplica).build())
                            .get();
                    AtomicLong endOffset = new AtomicLong(0);
                    AtomicBoolean stopped = new AtomicBoolean(false);
                    long startSeq = Utils.getRandomInt(0, 1024);
                    executor.submit(new Producer(stream, startSeq,
                            option.getMin(), option.getMax(), option.getInterval(),
                            endOffset, stopped,
                            lock, condition));
                    executor.submit(new TailReadConsumer(stream, startSeq, endOffset, stopped,
                            lock, condition));
                    executor.submit(new RepeatedReadConsumer(stream, startSeq, endOffset, stopped,
                            lock, condition));
                    lock.lock();
                    try {
                        while (!stopped.get()) {
                            condition.await();
                        }
                    } finally {
                        lock.unlock();
                    }
                }
            });
        }

    }
}
