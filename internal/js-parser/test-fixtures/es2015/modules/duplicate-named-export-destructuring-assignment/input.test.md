# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2015 > modules > duplicate-named-export-destructuring-assignment`

### `ast`

```javascript
JSRoot {
	body: [
		JSExportLocalDeclaration {
			exportKind: "value"
			specifiers: [
				JSExportLocalSpecifier {
					exported: JSIdentifier {
						name: "foo"
						loc: SourceLocation es2015/modules/duplicate-named-export-destructuring-assignment/input.js 1:9-1:12 (foo)
					}
					local: JSReferenceIdentifier {
						name: "foo"
						loc: SourceLocation es2015/modules/duplicate-named-export-destructuring-assignment/input.js 1:9-1:12 (foo)
					}
					loc: SourceLocation es2015/modules/duplicate-named-export-destructuring-assignment/input.js 1:9-1:12
				}
			]
			loc: SourceLocation es2015/modules/duplicate-named-export-destructuring-assignment/input.js 1:0-1:15
		}
		JSExportLocalDeclaration {
			exportKind: "value"
			declaration: JSVariableDeclarationStatement {
				declaration: JSVariableDeclaration {
					kind: "const"
					declarations: [
						JSVariableDeclarator {
							id: JSBindingObjectPattern {
								properties: [
									JSBindingObjectPatternProperty {
										key: JSStaticPropertyKey {
											value: JSIdentifier {
												name: "foo"
												loc: SourceLocation es2015/modules/duplicate-named-export-destructuring-assignment/input.js 2:15-2:18 (foo)
											}
											loc: SourceLocation es2015/modules/duplicate-named-export-destructuring-assignment/input.js 2:15-2:18
										}
										value: JSBindingAssignmentPattern {
											left: JSBindingIdentifier {
												name: "foo"
												loc: SourceLocation es2015/modules/duplicate-named-export-destructuring-assignment/input.js 2:15-2:18 (foo)
											}
											right: JSNumericLiteral {
												value: 1
												loc: SourceLocation es2015/modules/duplicate-named-export-destructuring-assignment/input.js 2:21-2:22
											}
											loc: SourceLocation es2015/modules/duplicate-named-export-destructuring-assignment/input.js 2:15-2:22
										}
										loc: SourceLocation es2015/modules/duplicate-named-export-destructuring-assignment/input.js 2:15-2:22
									}
								]
								loc: SourceLocation es2015/modules/duplicate-named-export-destructuring-assignment/input.js 2:13-2:24
							}
							init: JSReferenceIdentifier {
								name: "bar"
								loc: SourceLocation es2015/modules/duplicate-named-export-destructuring-assignment/input.js 2:27-2:30 (bar)
							}
							loc: SourceLocation es2015/modules/duplicate-named-export-destructuring-assignment/input.js 2:13-2:30
						}
					]
					loc: SourceLocation es2015/modules/duplicate-named-export-destructuring-assignment/input.js 2:7-2:31
				}
				loc: SourceLocation es2015/modules/duplicate-named-export-destructuring-assignment/input.js 2:7-2:31
			}
			loc: SourceLocation es2015/modules/duplicate-named-export-destructuring-assignment/input.js 2:0-2:31
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "module"
	syntax: []
	path: UIDPath<es2015/modules/duplicate-named-export-destructuring-assignment/input.js>
	loc: SourceLocation es2015/modules/duplicate-named-export-destructuring-assignment/input.js 1:0-3:0
}
```

### `diagnostics`

```

```