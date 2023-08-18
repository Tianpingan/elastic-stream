package kv

import (
	"context"
	"fmt"

	"go.uber.org/zap"

	"github.com/AutoMQ/pd/pkg/util/traceutil"
)

type LogAble interface {
	KV
	Logger() *zap.Logger
}

// Logger is a wrapper of KV that logs all operations.
type Logger struct {
	KV LogAble
}

func (l Logger) Get(ctx context.Context, k []byte) (v []byte, err error) {
	v, err = l.KV.Get(ctx, k)

	logger := l.logger()
	if logger.Core().Enabled(zap.DebugLevel) {
		logger = logger.With(traceutil.TraceLogField(ctx))
		logger.Debug("kv get", zap.ByteString("key", k), zap.Binary("value", v), zap.Error(err))
	}
	return
}

func (l Logger) BatchGet(ctx context.Context, keys [][]byte, inTxn bool) (kvs []KeyValue, err error) {
	kvs, err = l.KV.BatchGet(ctx, keys, inTxn)

	logger := l.logger()
	if logger.Core().Enabled(zap.DebugLevel) {
		logger = logger.With(traceutil.TraceLogField(ctx))
		fields := []zap.Field{
			zap.Bool("in-txn", inTxn),
			zap.Error(err),
		}
		for i, key := range keys {
			fields = append(fields, zap.ByteString(fmt.Sprintf("query-key-%d", i), key))
		}
		for i, kv := range kvs {
			fields = append(fields, zap.ByteString(fmt.Sprintf("key-%d", i), kv.Key), zap.Binary(fmt.Sprintf("value-%d", i), kv.Value))
		}
		logger.Debug("kv batch get", fields...)
	}
	return
}

func (l Logger) GetByRange(ctx context.Context, r Range, rev int64, limit int64, desc bool) (kvs []KeyValue, revision int64, more bool, err error) {
	kvs, revision, more, err = l.KV.GetByRange(ctx, r, rev, limit, desc)

	logger := l.logger()
	if logger.Core().Enabled(zap.DebugLevel) {
		logger = logger.With(traceutil.TraceLogField(ctx))
		fields := []zap.Field{
			zap.ByteString("start-key", r.StartKey),
			zap.ByteString("end-key", r.EndKey),
			zap.Int64("req-rev", rev),
			zap.Int64("limit", limit),
			zap.Bool("desc", desc),
			zap.Int64("resp-rev", revision),
			zap.Bool("more", more),
			zap.Error(err),
		}
		for i, kv := range kvs {
			fields = append(fields, zap.ByteString(fmt.Sprintf("key-%d", i), kv.Key), zap.Binary(fmt.Sprintf("value-%d", i), kv.Value))
		}
		logger.Debug("kv get by range", fields...)
	}
	return
}

func (l Logger) Watch(ctx context.Context, prefix []byte, rev int64, filter Filter) (w Watcher) {
	w = l.KV.Watch(ctx, prefix, rev, filter)

	logger := l.logger()
	if logger.Core().Enabled(zap.DebugLevel) {
		logger.Debug("kv watch", zap.ByteString("prefix", prefix), zap.Int64("rev", rev), traceutil.TraceLogField(ctx))
	}
	return
}

func (l Logger) Put(ctx context.Context, k, v []byte, prevKV bool, ttl int64) (prevV []byte, err error) {
	prevV, err = l.KV.Put(ctx, k, v, prevKV, ttl)

	logger := l.logger()
	if logger.Core().Enabled(zap.DebugLevel) {
		logger = logger.With(traceutil.TraceLogField(ctx))
		logger.Debug("kv put", zap.ByteString("key", k), zap.Binary("value", v), zap.Bool("prev-kv", prevKV), zap.Int64("ttl", ttl), zap.Binary("prev-value", prevV), zap.Error(err))
	}
	return
}

func (l Logger) BatchPut(ctx context.Context, kvs []KeyValue, prevKV bool, inTxn bool, ttl int64) (prevKVs []KeyValue, err error) {
	prevKVs, err = l.KV.BatchPut(ctx, kvs, prevKV, inTxn, ttl)

	logger := l.logger()
	if logger.Core().Enabled(zap.DebugLevel) {
		logger = logger.With(traceutil.TraceLogField(ctx))
		fields := []zap.Field{
			zap.Bool("prev-kv", prevKV),
			zap.Bool("in-txn", inTxn),
			zap.Int64("ttl", ttl),
			zap.Error(err),
		}
		for i, kv := range kvs {
			fields = append(fields, zap.ByteString(fmt.Sprintf("key-%d", i), kv.Key), zap.Binary(fmt.Sprintf("value-%d", i), kv.Value))
		}
		for i, kv := range prevKVs {
			fields = append(fields, zap.ByteString(fmt.Sprintf("prev-key-%d", i), kv.Key), zap.Binary(fmt.Sprintf("prev-value-%d", i), kv.Value))
		}
		logger.Debug("kv batch put", fields...)
	}
	return
}

func (l Logger) Delete(ctx context.Context, k []byte, prevKV bool) (prevV []byte, err error) {
	prevV, err = l.KV.Delete(ctx, k, prevKV)

	logger := l.logger()
	if logger.Core().Enabled(zap.DebugLevel) {
		logger = logger.With(traceutil.TraceLogField(ctx))
		logger.Debug("kv delete", zap.ByteString("key", k), zap.Bool("prev-kv", prevKV), zap.Binary("prev-value", prevV), zap.Error(err))
	}
	return
}

func (l Logger) BatchDelete(ctx context.Context, keys [][]byte, prevKV bool, inTxn bool) (prevKVs []KeyValue, err error) {
	prevKVs, err = l.KV.BatchDelete(ctx, keys, prevKV, inTxn)

	logger := l.logger()
	if logger.Core().Enabled(zap.DebugLevel) {
		logger = logger.With(traceutil.TraceLogField(ctx))
		fields := []zap.Field{
			zap.Bool("prev-kv", prevKV),
			zap.Bool("in-txn", inTxn),
			zap.Error(err),
		}
		for i, k := range keys {
			fields = append(fields, zap.ByteString(fmt.Sprintf("key-%d", i), k))
		}
		for i, kv := range prevKVs {
			fields = append(fields, zap.ByteString(fmt.Sprintf("prev-key-%d", i), kv.Key), zap.Binary(fmt.Sprintf("prev-value-%d", i), kv.Value))
		}
		logger.Debug("kv batch delete", fields...)
	}
	return
}

func (l Logger) DeleteByRange(ctx context.Context, r Range) (int, error) {
	deleted, err := l.KV.DeleteByRange(ctx, r)

	logger := l.logger()
	if logger.Core().Enabled(zap.DebugLevel) {
		logger = logger.With(traceutil.TraceLogField(ctx))
		logger.Debug("kv delete by range", zap.ByteString("start-key", r.StartKey), zap.ByteString("end-key", r.EndKey), zap.Int("deleted", deleted), zap.Error(err))
	}
	return deleted, err
}

func (l Logger) ExecInTxn(ctx context.Context, f func(kv BasicKV) error) error {
	return l.KV.ExecInTxn(ctx, f)
}

func (l Logger) GetPrefixRangeEnd(prefix []byte) []byte {
	return l.KV.GetPrefixRangeEnd(prefix)
}

func (l Logger) logger() *zap.Logger {
	if l.KV.Logger() != nil {
		return l.KV.Logger()
	}
	return zap.NewNop()
}
