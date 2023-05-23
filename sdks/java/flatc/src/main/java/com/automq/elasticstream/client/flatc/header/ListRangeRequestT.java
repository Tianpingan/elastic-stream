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

public class ListRangeRequestT {
  private int timeoutMs;
  private com.automq.elasticstream.client.flatc.header.ListRangeCriteriaT criteria;

  public int getTimeoutMs() { return timeoutMs; }

  public void setTimeoutMs(int timeoutMs) { this.timeoutMs = timeoutMs; }

  public com.automq.elasticstream.client.flatc.header.ListRangeCriteriaT getCriteria() { return criteria; }

  public void setCriteria(com.automq.elasticstream.client.flatc.header.ListRangeCriteriaT criteria) { this.criteria = criteria; }


  public ListRangeRequestT() {
    this.timeoutMs = 0;
    this.criteria = null;
  }
}

