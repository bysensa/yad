import 'package:app/app.module.dart';
import 'package:app/app/db/provider.dart';
import 'package:app/workspace/model.dart';
import 'package:mobx/mobx.dart';
import 'package:oxidized/oxidized.dart';

class WorkspaceRepository {
  ObservableStream<void>? _collectionStream;
  late final dbProvider = AppModule.dbProvider();

  Future<void> setupListener() async {
    _collectionStream ??= await dbProvider()
        .map((db) => db.workspaces.watchLazy())
        .map((stream) => ObservableStream(stream))
        .unwrap();
    _collectionStream!.value;
  }
}
