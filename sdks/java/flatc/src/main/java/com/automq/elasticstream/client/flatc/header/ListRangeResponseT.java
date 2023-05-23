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

public class ListRangeResponseT {
  private com.automq.elasticstream.client.flatc.header.StatusT status;
  private int throttleTimeMs;
  private com.automq.elasticstream.client.flatc.header.RangeT[] ranges;

  public com.automq.elasticstream.client.flatc.header.StatusT getStatus() { return status; }

  public void setStatus(com.automq.elasticstream.client.flatc.header.StatusT status) { this.status = status; }

  public int getThrottleTimeMs() { return throttleTimeMs; }

  public void setThrottleTimeMs(int throttleTimeMs) { this.throttleTimeMs = throttleTimeMs; }

  public com.automq.elasticstream.client.flatc.header.RangeT[] getRanges() { return ranges; }

  public void setRanges(com.automq.elasticstream.client.flatc.header.RangeT[] ranges) { this.ranges = ranges; }


  public ListRangeResponseT() {
    this.status = null;
    this.throttleTimeMs = 0;
    this.ranges = null;
  }
}

