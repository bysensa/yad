import 'package:app/app/async_provider.dart';
import 'package:flutter/foundation.dart';
import 'package:isar/isar.dart';
import 'package:oxidized/oxidized.dart';

typedef Db = Isar;

class DbProvider with AsyncProvider<Db> {
  final List<CollectionSchema<dynamic>> _schemas = [];

  void registerSchema(CollectionSchema<dynamic> schema) => _schemas.add(schema);

  @override
  Future<Result<Db, Object>> provides() {
    return Result.asyncOf(
      () => Db.open(
        _schemas,
        inspector: kDebugMode,
      ),
    );
  }
}
