// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'model.dart';

// **************************************************************************
// IsarCollectionGenerator
// **************************************************************************

// coverage:ignore-file
// ignore_for_file: duplicate_ignore, non_constant_identifier_names, constant_identifier_names, invalid_use_of_protected_member, unnecessary_cast, prefer_const_constructors, lines_longer_than_80_chars, require_trailing_commas, inference_failure_on_function_invocation, unnecessary_parenthesis, unnecessary_raw_strings, unnecessary_null_checks, join_return_with_assignment, prefer_final_locals, avoid_js_rounded_ints, avoid_positional_boolean_parameters

extension GetWorkspaceCollection on Isar {
  IsarCollection<Workspace> get workspaces => this.collection();
}

const WorkspaceSchema = CollectionSchema(
  name: r'Workspace',
  id: 264710109339051155,
  properties: {},
  estimateSize: _workspaceEstimateSize,
  serialize: _workspaceSerialize,
  deserialize: _workspaceDeserialize,
  deserializeProp: _workspaceDeserializeProp,
  idName: r'id',
  indexes: {},
  links: {},
  embeddedSchemas: {},
  getId: _workspaceGetId,
  getLinks: _workspaceGetLinks,
  attach: _workspaceAttach,
  version: '3.0.5',
);

int _workspaceEstimateSize(
  Workspace object,
  List<int> offsets,
  Map<Type, List<int>> allOffsets,
) {
  var bytesCount = offsets.last;
  return bytesCount;
}

void _workspaceSerialize(
  Workspace object,
  IsarWriter writer,
  List<int> offsets,
  Map<Type, List<int>> allOffsets,
) {}
Workspace _workspaceDeserialize(
  Id id,
  IsarReader reader,
  List<int> offsets,
  Map<Type, List<int>> allOffsets,
) {
  final object = Workspace();
  object.id = id;
  return object;
}

P _workspaceDeserializeProp<P>(
  IsarReader reader,
  int propertyId,
  int offset,
  Map<Type, List<int>> allOffsets,
) {
  switch (propertyId) {
    default:
      throw IsarError('Unknown property with id $propertyId');
  }
}

Id _workspaceGetId(Workspace object) {
  return object.id;
}

List<IsarLinkBase<dynamic>> _workspaceGetLinks(Workspace object) {
  return [];
}

void _workspaceAttach(IsarCollection<dynamic> col, Id id, Workspace object) {
  object.id = id;
}

extension WorkspaceQueryWhereSort
    on QueryBuilder<Workspace, Workspace, QWhere> {
  QueryBuilder<Workspace, Workspace, QAfterWhere> anyId() {
    return QueryBuilder.apply(this, (query) {
      return query.addWhereClause(const IdWhereClause.any());
    });
  }
}

extension WorkspaceQueryWhere
    on QueryBuilder<Workspace, Workspace, QWhereClause> {
  QueryBuilder<Workspace, Workspace, QAfterWhereClause> idEqualTo(Id id) {
    return QueryBuilder.apply(this, (query) {
      return query.addWhereClause(IdWhereClause.between(
        lower: id,
        upper: id,
      ));
    });
  }

  QueryBuilder<Workspace, Workspace, QAfterWhereClause> idNotEqualTo(Id id) {
    return QueryBuilder.apply(this, (query) {
      if (query.whereSort == Sort.asc) {
        return query
            .addWhereClause(
              IdWhereClause.lessThan(upper: id, includeUpper: false),
            )
            .addWhereClause(
              IdWhereClause.greaterThan(lower: id, includeLower: false),
            );
      } else {
        return query
            .addWhereClause(
              IdWhereClause.greaterThan(lower: id, includeLower: false),
            )
            .addWhereClause(
              IdWhereClause.lessThan(upper: id, includeUpper: false),
            );
      }
    });
  }

  QueryBuilder<Workspace, Workspace, QAfterWhereClause> idGreaterThan(Id id,
      {bool include = false}) {
    return QueryBuilder.apply(this, (query) {
      return query.addWhereClause(
        IdWhereClause.greaterThan(lower: id, includeLower: include),
      );
    });
  }

  QueryBuilder<Workspace, Workspace, QAfterWhereClause> idLessThan(Id id,
      {bool include = false}) {
    return QueryBuilder.apply(this, (query) {
      return query.addWhereClause(
        IdWhereClause.lessThan(upper: id, includeUpper: include),
      );
    });
  }

  QueryBuilder<Workspace, Workspace, QAfterWhereClause> idBetween(
    Id lowerId,
    Id upperId, {
    bool includeLower = true,
    bool includeUpper = true,
  }) {
    return QueryBuilder.apply(this, (query) {
      return query.addWhereClause(IdWhereClause.between(
        lower: lowerId,
        includeLower: includeLower,
        upper: upperId,
        includeUpper: includeUpper,
      ));
    });
  }
}

extension WorkspaceQueryFilter
    on QueryBuilder<Workspace, Workspace, QFilterCondition> {
  QueryBuilder<Workspace, Workspace, QAfterFilterCondition> idEqualTo(
      Id value) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(FilterCondition.equalTo(
        property: r'id',
        value: value,
      ));
    });
  }

  QueryBuilder<Workspace, Workspace, QAfterFilterCondition> idGreaterThan(
    Id value, {
    bool include = false,
  }) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(FilterCondition.greaterThan(
        include: include,
        property: r'id',
        value: value,
      ));
    });
  }

  QueryBuilder<Workspace, Workspace, QAfterFilterCondition> idLessThan(
    Id value, {
    bool include = false,
  }) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(FilterCondition.lessThan(
        include: include,
        property: r'id',
        value: value,
      ));
    });
  }

  QueryBuilder<Workspace, Workspace, QAfterFilterCondition> idBetween(
    Id lower,
    Id upper, {
    bool includeLower = true,
    bool includeUpper = true,
  }) {
    return QueryBuilder.apply(this, (query) {
      return query.addFilterCondition(FilterCondition.between(
        property: r'id',
        lower: lower,
        includeLower: includeLower,
        upper: upper,
        includeUpper: includeUpper,
      ));
    });
  }
}

extension WorkspaceQueryObject
    on QueryBuilder<Workspace, Workspace, QFilterCondition> {}

extension WorkspaceQueryLinks
    on QueryBuilder<Workspace, Workspace, QFilterCondition> {}

extension WorkspaceQuerySortBy on QueryBuilder<Workspace, Workspace, QSortBy> {}

extension WorkspaceQuerySortThenBy
    on QueryBuilder<Workspace, Workspace, QSortThenBy> {
  QueryBuilder<Workspace, Workspace, QAfterSortBy> thenById() {
    return QueryBuilder.apply(this, (query) {
      return query.addSortBy(r'id', Sort.asc);
    });
  }

  QueryBuilder<Workspace, Workspace, QAfterSortBy> thenByIdDesc() {
    return QueryBuilder.apply(this, (query) {
      return query.addSortBy(r'id', Sort.desc);
    });
  }
}

extension WorkspaceQueryWhereDistinct
    on QueryBuilder<Workspace, Workspace, QDistinct> {}

extension WorkspaceQueryProperty
    on QueryBuilder<Workspace, Workspace, QQueryProperty> {
  QueryBuilder<Workspace, int, QQueryOperations> idProperty() {
    return QueryBuilder.apply(this, (query) {
      return query.addPropertyName(r'id');
    });
  }
}
