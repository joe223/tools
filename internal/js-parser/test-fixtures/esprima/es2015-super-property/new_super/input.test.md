# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > es2015-super-property > new_super`

### `ast`

```javascript
JSRoot {
	body: [
		JSClassDeclaration {
			id: JSBindingIdentifier {
				name: "A"
				loc: SourceLocation esprima/es2015-super-property/new_super/input.js 1:6-1:7 (A)
			}
			meta: JSClassHead {
				body: [
					JSClassMethod {
						kind: "method"
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "foo"
								loc: SourceLocation esprima/es2015-super-property/new_super/input.js 2:4-2:7 (foo)
							}
							loc: SourceLocation esprima/es2015-super-property/new_super/input.js 2:4-2:7
						}
						meta: JSClassPropertyMeta {
							abstract: false
							optional: false
							readonly: false
							static: false
							loc: SourceLocation esprima/es2015-super-property/new_super/input.js 2:4-2:7
							start: Position 2:4
						}
						body: JSBlockStatement {
							body: [
								JSExpressionStatement {
									expression: JSNewExpression {
										arguments: []
										callee: JSMemberExpression {
											object: JSSuper {
												loc: SourceLocation esprima/es2015-super-property/new_super/input.js 3:12-3:17
											}
											property: JSStaticMemberProperty {
												value: JSIdentifier {
													name: "bar"
													loc: SourceLocation esprima/es2015-super-property/new_super/input.js 3:18-3:21 (bar)
												}
												loc: SourceLocation esprima/es2015-super-property/new_super/input.js 3:18-3:21 (bar)
											}
											loc: SourceLocation esprima/es2015-super-property/new_super/input.js 3:12-3:21
										}
										loc: SourceLocation esprima/es2015-super-property/new_super/input.js 3:8-3:23
									}
									loc: SourceLocation esprima/es2015-super-property/new_super/input.js 3:8-3:23
								}
							]
							directives: []
							loc: SourceLocation esprima/es2015-super-property/new_super/input.js 2:10-4:5
						}
						head: JSFunctionHead {
							async: false
							generator: false
							hasHoistedVars: false
							params: []
							loc: SourceLocation esprima/es2015-super-property/new_super/input.js 2:7-2:9
						}
						loc: SourceLocation esprima/es2015-super-property/new_super/input.js 2:4-4:5
					}
				]
				superClass: JSReferenceIdentifier {
					name: "B"
					loc: SourceLocation esprima/es2015-super-property/new_super/input.js 1:16-1:17 (B)
				}
				loc: SourceLocation esprima/es2015-super-property/new_super/input.js 1:0-5:1
			}
			loc: SourceLocation esprima/es2015-super-property/new_super/input.js 1:0-5:1
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/es2015-super-property/new_super/input.js>
	loc: SourceLocation esprima/es2015-super-property/new_super/input.js 1:0-6:0
}
```

### `diagnostics`

```

```