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
public final class FetchResponse extends Table {
  public static void ValidateVersion() { Constants.FLATBUFFERS_23_3_3(); }
  public static FetchResponse getRootAsFetchResponse(ByteBuffer _bb) { return getRootAsFetchResponse(_bb, new FetchResponse()); }
  public static FetchResponse getRootAsFetchResponse(ByteBuffer _bb, FetchResponse obj) { _bb.order(ByteOrder.LITTLE_ENDIAN); return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __reset(_i, _bb); }
  public FetchResponse __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public com.automq.elasticstream.client.flatc.header.Status status() { return status(new com.automq.elasticstream.client.flatc.header.Status()); }
  public com.automq.elasticstream.client.flatc.header.Status status(com.automq.elasticstream.client.flatc.header.Status obj) { int o = __offset(4); return o != 0 ? obj.__assign(__indirect(o + bb_pos), bb) : null; }
  public com.automq.elasticstream.client.flatc.header.FetchResultEntry entries(int j) { return entries(new com.automq.elasticstream.client.flatc.header.FetchResultEntry(), j); }
  public com.automq.elasticstream.client.flatc.header.FetchResultEntry entries(com.automq.elasticstream.client.flatc.header.FetchResultEntry obj, int j) { int o = __offset(6); return o != 0 ? obj.__assign(__indirect(__vector(o) + j * 4), bb) : null; }
  public int entriesLength() { int o = __offset(6); return o != 0 ? __vector_len(o) : 0; }
  public com.automq.elasticstream.client.flatc.header.FetchResultEntry.Vector entriesVector() { return entriesVector(new com.automq.elasticstream.client.flatc.header.FetchResultEntry.Vector()); }
  public com.automq.elasticstream.client.flatc.header.FetchResultEntry.Vector entriesVector(com.automq.elasticstream.client.flatc.header.FetchResultEntry.Vector obj) { int o = __offset(6); return o != 0 ? obj.__assign(__vector(o), 4, bb) : null; }
  public int throttleTimeMs() { int o = __offset(8); return o != 0 ? bb.getInt(o + bb_pos) : 0; }

  public static int createFetchResponse(FlatBufferBuilder builder,
      int statusOffset,
      int entriesOffset,
      int throttleTimeMs) {
    builder.startTable(3);
    FetchResponse.addThrottleTimeMs(builder, throttleTimeMs);
    FetchResponse.addEntries(builder, entriesOffset);
    FetchResponse.addStatus(builder, statusOffset);
    return FetchResponse.endFetchResponse(builder);
  }

  public static void startFetchResponse(FlatBufferBuilder builder) { builder.startTable(3); }
  public static void addStatus(FlatBufferBuilder builder, int statusOffset) { builder.addOffset(0, statusOffset, 0); }
  public static void addEntries(FlatBufferBuilder builder, int entriesOffset) { builder.addOffset(1, entriesOffset, 0); }
  public static int createEntriesVector(FlatBufferBuilder builder, int[] data) { builder.startVector(4, data.length, 4); for (int i = data.length - 1; i >= 0; i--) builder.addOffset(data[i]); return builder.endVector(); }
  public static void startEntriesVector(FlatBufferBuilder builder, int numElems) { builder.startVector(4, numElems, 4); }
  public static void addThrottleTimeMs(FlatBufferBuilder builder, int throttleTimeMs) { builder.addInt(2, throttleTimeMs, 0); }
  public static int endFetchResponse(FlatBufferBuilder builder) {
    int o = builder.endTable();
    builder.required(o, 4);  // status
    return o;
  }

  public static final class Vector extends BaseVector {
    public Vector __assign(int _vector, int _element_size, ByteBuffer _bb) { __reset(_vector, _element_size, _bb); return this; }

    public FetchResponse get(int j) { return get(new FetchResponse(), j); }
    public FetchResponse get(FetchResponse obj, int j) {  return obj.__assign(__indirect(__element(j), bb), bb); }
  }
  public FetchResponseT unpack() {
    FetchResponseT _o = new FetchResponseT();
    unpackTo(_o);
    return _o;
  }
  public void unpackTo(FetchResponseT _o) {
    if (status() != null) _o.setStatus(status().unpack());
    else _o.setStatus(null);
    com.automq.elasticstream.client.flatc.header.FetchResultEntryT[] _oEntries = new com.automq.elasticstream.client.flatc.header.FetchResultEntryT[entriesLength()];
    for (int _j = 0; _j < entriesLength(); ++_j) {_oEntries[_j] = (entries(_j) != null ? entries(_j).unpack() : null);}
    _o.setEntries(_oEntries);
    int _oThrottleTimeMs = throttleTimeMs();
    _o.setThrottleTimeMs(_oThrottleTimeMs);
  }
  public static int pack(FlatBufferBuilder builder, FetchResponseT _o) {
    if (_o == null) return 0;
    int _status = _o.getStatus() == null ? 0 : com.automq.elasticstream.client.flatc.header.Status.pack(builder, _o.getStatus());
    int _entries = 0;
    if (_o.getEntries() != null) {
      int[] __entries = new int[_o.getEntries().length];
      int _j = 0;
      for (com.automq.elasticstream.client.flatc.header.FetchResultEntryT _e : _o.getEntries()) { __entries[_j] = com.automq.elasticstream.client.flatc.header.FetchResultEntry.pack(builder, _e); _j++;}
      _entries = createEntriesVector(builder, __entries);
    }
    return createFetchResponse(
      builder,
      _status,
      _entries,
      _o.getThrottleTimeMs());
  }
}

