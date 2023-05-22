package sdk.elastic.stream.jni;
import java.util.concurrent.CompletableFuture;
public class Stream extends ElasticStreamObject {
    public Stream(long ptr) {
        this.ptr = ptr;
    }
    public CompletableFuture<Long> minOffset() {
        CompletableFuture<Long> future = new CompletableFuture<>();
        minOffset(this.ptr, future);
        return future;
    }
    public CompletableFuture<Long> maxOffset() {
        CompletableFuture<Long> future = new CompletableFuture<>();
        maxOffset(this.ptr, future);
        return future;
    }
    public CompletableFuture<Long> append(byte[] data) {
        CompletableFuture<Long> future = new CompletableFuture<>();
        append(this.ptr, data, future);
        return future;
    }
    public CompletableFuture<byte[]> read(long offset, int limit, int max_bytes) {
        CompletableFuture<byte[]> future = new CompletableFuture<>();
        read(this.ptr, offset, limit, max_bytes, future);
        return future;
    }
    private native void minOffset(long ptr, CompletableFuture<Long> future);
    private native void maxOffset(long ptr, CompletableFuture<Long> future);
    private native void append(long ptr, byte[] data, CompletableFuture<Long> future);
    private native void read(long ptr, long offset, int limit, int max_bytes, CompletableFuture<byte[]> future);
    private native long freeStream(long ptr);
    @Override
    public void close() {
        freeStream(this.ptr);
    }
    
}
