import 'package:app/app/db/provider.dart';
import 'package:managed/managed.dart';

abstract class AppModule {
  static final dbProvider = Manage(DbProvider.new);
}
