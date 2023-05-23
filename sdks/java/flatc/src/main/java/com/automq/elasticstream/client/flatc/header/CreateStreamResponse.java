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

@SuppressWarnings("unused")
public final class CreateStreamResponse extends Table {
  public static void ValidateVersion() { Constants.FLATBUFFERS_23_3_3(); }
  public static CreateStreamResponse getRootAsCreateStreamResponse(ByteBuffer _bb) { return getRootAsCreateStreamResponse(_bb, new CreateStreamResponse()); }
  public static CreateStreamResponse getRootAsCreateStreamResponse(ByteBuffer _bb, CreateStreamResponse obj) { _bb.order(ByteOrder.LITTLE_ENDIAN); return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __reset(_i, _bb); }
  public CreateStreamResponse __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public com.automq.elasticstream.client.flatc.header.Status status() { return status(new com.automq.elasticstream.client.flatc.header.Status()); }
  public com.automq.elasticstream.client.flatc.header.Status status(com.automq.elasticstream.client.flatc.header.Status obj) { int o = __offset(4); return o != 0 ? obj.__assign(__indirect(o + bb_pos), bb) : null; }
  public com.automq.elasticstream.client.flatc.header.Stream stream() { return stream(new com.automq.elasticstream.client.flatc.header.Stream()); }
  public com.automq.elasticstream.client.flatc.header.Stream stream(com.automq.elasticstream.client.flatc.header.Stream obj) { int o = __offset(6); return o != 0 ? obj.__assign(__indirect(o + bb_pos), bb) : null; }
  public int throttleTimeMs() { int o = __offset(8); return o != 0 ? bb.getInt(o + bb_pos) : 0; }

  public static int createCreateStreamResponse(FlatBufferBuilder builder,
      int statusOffset,
      int streamOffset,
      int throttleTimeMs) {
    builder.startTable(3);
    CreateStreamResponse.addThrottleTimeMs(builder, throttleTimeMs);
    CreateStreamResponse.addStream(builder, streamOffset);
    CreateStreamResponse.addStatus(builder, statusOffset);
    return CreateStreamResponse.endCreateStreamResponse(builder);
  }

  public static void startCreateStreamResponse(FlatBufferBuilder builder) { builder.startTable(3); }
  public static void addStatus(FlatBufferBuilder builder, int statusOffset) { builder.addOffset(0, statusOffset, 0); }
  public static void addStream(FlatBufferBuilder builder, int streamOffset) { builder.addOffset(1, streamOffset, 0); }
  public static void addThrottleTimeMs(FlatBufferBuilder builder, int throttleTimeMs) { builder.addInt(2, throttleTimeMs, 0); }
  public static int endCreateStreamResponse(FlatBufferBuilder builder) {
    int o = builder.endTable();
    builder.required(o, 4);  // status
    return o;
  }

  public static final class Vector extends BaseVector {
    public Vector __assign(int _vector, int _element_size, ByteBuffer _bb) { __reset(_vector, _element_size, _bb); return this; }

    public CreateStreamResponse get(int j) { return get(new CreateStreamResponse(), j); }
    public CreateStreamResponse get(CreateStreamResponse obj, int j) {  return obj.__assign(__indirect(__element(j), bb), bb); }
  }
  public CreateStreamResponseT unpack() {
    CreateStreamResponseT _o = new CreateStreamResponseT();
    unpackTo(_o);
    return _o;
  }
  public void unpackTo(CreateStreamResponseT _o) {
    if (status() != null) _o.setStatus(status().unpack());
    else _o.setStatus(null);
    if (stream() != null) _o.setStream(stream().unpack());
    else _o.setStream(null);
    int _oThrottleTimeMs = throttleTimeMs();
    _o.setThrottleTimeMs(_oThrottleTimeMs);
  }
  public static int pack(FlatBufferBuilder builder, CreateStreamResponseT _o) {
    if (_o == null) return 0;
    int _status = _o.getStatus() == null ? 0 : com.automq.elasticstream.client.flatc.header.Status.pack(builder, _o.getStatus());
    int _stream = _o.getStream() == null ? 0 : com.automq.elasticstream.client.flatc.header.Stream.pack(builder, _o.getStream());
    return createCreateStreamResponse(
      builder,
      _status,
      _stream,
      _o.getThrottleTimeMs());
  }
}

