package com.automq.elasticstream.client.jni;
import java.nio.ByteBuffer;
import java.util.concurrent.CompletableFuture;
public class Stream extends ElasticStreamObject {
    public Stream(long ptr) {
        this.ptr = ptr;
    }
    public CompletableFuture<Long> startOffset() {
        CompletableFuture<Long> future = new CompletableFuture<>();
        startOffset(this.ptr, future);
        return future;
    }
    public CompletableFuture<Long> nextOffset() {
        CompletableFuture<Long> future = new CompletableFuture<>();
        nextOffset(this.ptr, future);
        return future;
    }
    public CompletableFuture<Long> append(ByteBuffer data) {
        CompletableFuture<Long> future = new CompletableFuture<>();
        append(this.ptr, data, future);
        return future;
    }
    public CompletableFuture<ByteBuffer> read(long start_offset, long end_offset, int batch_max_bytes) {
        CompletableFuture<ByteBuffer> future = new CompletableFuture<>();
        read(this.ptr, start_offset, end_offset, batch_max_bytes, future);
        return future;
    }
    public CompletableFuture<Void> asyncClose() {
        CompletableFuture<Void> future = new CompletableFuture<>();
        asyncClose(this.ptr, future);
        return future;
    }

    public CompletableFuture<Void> trim(long newStartOffset) {
        CompletableFuture<Void> future = new CompletableFuture<>();
        trim(this.ptr, newStartOffset, future);
        return future;
    }

    public CompletableFuture<Void> delete() {
        CompletableFuture<Void> future = new CompletableFuture<>();
        delete(this.ptr, future);
        return future;
    }

    private native void startOffset(long ptr, CompletableFuture<Long> future);
    private native void nextOffset(long ptr, CompletableFuture<Long> future);
    private native void append(long ptr, ByteBuffer data, CompletableFuture<Long> future);
    private native void read(long ptr, long start_offset, long end_offset, int batch_max_bytes, CompletableFuture<ByteBuffer> future);
    private native void asyncClose(long ptr, CompletableFuture<Void> future);
    private native long freeStream(long ptr);
    private native void trim(long ptr, long new_start_offset, CompletableFuture<Void> future);
    private native void delete(long ptr, CompletableFuture<Void> future);
    @Override
    public void close() {
        freeStream(this.ptr);
    }

}
