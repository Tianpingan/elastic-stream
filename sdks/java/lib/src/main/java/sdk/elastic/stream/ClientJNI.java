import java.nio.ByteBuffer;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.ExecutionException;
public class ClientJNI {
    
    long ptr;
    public ClientJNI() {
        this.ptr = getClient();
    }
    private native long getClient();
    private native void freeClient(long ptr);
    private native void createStream(long ptr);
    public void createStream() {
        createStream(this.ptr);
    }
    private native void asyncCreateStream(long ptr, CompletableFuture<byte[]> future);
    public CompletableFuture<byte[]> asyncCreateStream() {
        CompletableFuture<byte[]> future = new CompletableFuture<>();
        asyncCreateStream(this.ptr, future);
        return future;
    }

    static {
        System.load("/home/tpa/tianpingan/elastic-stream/target/release/libfront_end_sdk.so");
        // System.loadLibrary("front_end_sdk");
    }
    
    public static void main(String[] args) throws Exception{
        ClientJNI client = new ClientJNI();
        client.createStream();
        CompletableFuture<byte[]> future = client.asyncCreateStream();
        future.thenAccept((result) -> {
            for (int i = 0; i < result.length; i++) {
                System.out.println("result[" + i + "]: " + result[i]);
            }
        });
        client.finalize();
    } 
    @Override
    protected void finalize() {
        System.out.println("in finalize()");
        freeClient(this.ptr);
    }
}
