import java.nio.ByteBuffer;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.ExecutionException;
public class ClientJNI {
    private static native void asyncComputation(byte[] input, CompletableFuture<byte[]> future);
    static {
        System.load("/home/tpa/tianpingan/elastic-stream/target/release/libfront_end_sdk.so");
        // System.loadLibrary("front_end_sdk");
    }
    public static void main(String[] args) throws Exception{
        CompletableFuture<byte[]> future = ClientJNI.AsyncFunc("abcd".getBytes());
        future.thenAccept((result) -> {
            System.out.println("after 1 sec");
            for (int i = 0; i < result.length; i++) {
                System.out.println("result[" + i + "]: " + result[i]);
            }
        });
        System.out.println("end of main");
        Thread.sleep(1000 * 2);
    } 

    public static CompletableFuture<byte[]> AsyncFunc(byte[] input) {
        CompletableFuture<byte[]> future = new CompletableFuture<>();
        ClientJNI.asyncComputation(input, future);
        return future;
    }
}
