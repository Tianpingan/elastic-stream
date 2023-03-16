// automatically generated by the FlatBuffers compiler, do not modify

package header;

import com.google.flatbuffers.BaseVector;
import com.google.flatbuffers.BooleanVector;
import com.google.flatbuffers.ByteVector;
import com.google.flatbuffers.Constants;
import com.google.flatbuffers.DoubleVector;
import com.google.flatbuffers.FlatBufferBuilder;
import com.google.flatbuffers.FloatVector;
import com.google.flatbuffers.IntVector;
import com.google.flatbuffers.LongVector;
import com.google.flatbuffers.ShortVector;
import com.google.flatbuffers.StringVector;
import com.google.flatbuffers.Struct;
import com.google.flatbuffers.Table;
import com.google.flatbuffers.UnionVector;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;

public class ListRangesResponseT {
  private int throttleTimeMs;
  private header.ListRangesResultT[] listResponses;
  private short errorCode;
  private String errorMessage;

  public int getThrottleTimeMs() { return throttleTimeMs; }

  public void setThrottleTimeMs(int throttleTimeMs) { this.throttleTimeMs = throttleTimeMs; }

  public header.ListRangesResultT[] getListResponses() { return listResponses; }

  public void setListResponses(header.ListRangesResultT[] listResponses) { this.listResponses = listResponses; }

  public short getErrorCode() { return errorCode; }

  public void setErrorCode(short errorCode) { this.errorCode = errorCode; }

  public String getErrorMessage() { return errorMessage; }

  public void setErrorMessage(String errorMessage) { this.errorMessage = errorMessage; }


  public ListRangesResponseT() {
    this.throttleTimeMs = 0;
    this.listResponses = null;
    this.errorCode = 0;
    this.errorMessage = null;
  }
}

