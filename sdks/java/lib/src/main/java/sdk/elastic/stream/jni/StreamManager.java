package sdk.elastic.stream.jni;
import java.util.concurrent.CompletableFuture;
public class StreamManager extends ElasticStreamObject {
    public StreamManager() {
        this.ptr = getStreamManager();
    }
    public CompletableFuture<Stream> create(int replica) {
        CompletableFuture<Stream> future = new CompletableFuture<>(); 
        create(this.ptr, replica, future);
        return future;
    }
    public CompletableFuture<Stream> open(long id) {
        CompletableFuture<Stream> future = new CompletableFuture<>();
        open(this.ptr, id, future);
        return future;
    }
    private native void create(long ptr, int replica, CompletableFuture<Stream> future);
    private native void open(long ptr, long id, CompletableFuture<Stream> future);
    private native long getStreamManager();
    private native void freeStreamManager(long ptr);
    @Override
    public void close() {
        freeStreamManager(this.ptr);
    }
}
