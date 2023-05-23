// automatically generated by the FlatBuffers compiler, do not modify

package com.automq.elasticstream.client.flatc.header;

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

public class CreateStreamResponseT {
  private com.automq.elasticstream.client.flatc.header.StatusT status;
  private com.automq.elasticstream.client.flatc.header.StreamT stream;
  private int throttleTimeMs;

  public com.automq.elasticstream.client.flatc.header.StatusT getStatus() { return status; }

  public void setStatus(com.automq.elasticstream.client.flatc.header.StatusT status) { this.status = status; }

  public com.automq.elasticstream.client.flatc.header.StreamT getStream() { return stream; }

  public void setStream(com.automq.elasticstream.client.flatc.header.StreamT stream) { this.stream = stream; }

  public int getThrottleTimeMs() { return throttleTimeMs; }

  public void setThrottleTimeMs(int throttleTimeMs) { this.throttleTimeMs = throttleTimeMs; }


  public CreateStreamResponseT() {
    this.status = null;
    this.stream = null;
    this.throttleTimeMs = 0;
  }
}

