import 'package:async/async.dart' hide Result;
import 'package:flutter/foundation.dart';
import 'package:oxidized/oxidized.dart';

abstract class AsyncProvider<T extends Object> {
  AsyncMemoizer<Result<T, Object>> _memoizer = AsyncMemoizer();

  Future<Result<T, Object>> call() => _memoizer.runOnce(provides);

  @protected
  Future<Result<T, Object>> provides();

  bool get provided => _memoizer.hasRun;

  void reset() => _memoizer = AsyncMemoizer();
}
