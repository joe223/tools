# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `experimental > class-properties > super`

### `ast`

```javascript
JSRoot {
	body: [
		JSClassDeclaration {
			id: JSBindingIdentifier {
				name: "Fails"
				loc: SourceLocation experimental/class-properties/super/input.js 1:6-1:11 (Fails)
			}
			meta: JSClassHead {
				body: [
					JSClassProperty {
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "c"
								loc: SourceLocation experimental/class-properties/super/input.js 2:2-2:3 (c)
							}
							loc: SourceLocation experimental/class-properties/super/input.js 2:2-2:3
						}
						meta: JSClassPropertyMeta {
							abstract: false
							optional: false
							readonly: false
							static: false
							loc: SourceLocation experimental/class-properties/super/input.js 2:2-2:3
							start: Position 2:2
						}
						value: JSCallExpression {
							arguments: []
							callee: JSMemberExpression {
								object: JSSuper {
									loc: SourceLocation experimental/class-properties/super/input.js 2:6-2:11
								}
								property: JSStaticMemberProperty {
									value: JSIdentifier {
										name: "c"
										loc: SourceLocation experimental/class-properties/super/input.js 2:12-2:13 (c)
									}
									loc: SourceLocation experimental/class-properties/super/input.js 2:12-2:13 (c)
								}
								loc: SourceLocation experimental/class-properties/super/input.js 2:6-2:13
							}
							loc: SourceLocation experimental/class-properties/super/input.js 2:6-2:15
						}
						loc: SourceLocation experimental/class-properties/super/input.js 2:2-2:16
					}
				]
				superClass: JSClassExpression {
					meta: JSClassHead {
						body: [
							JSClassMethod {
								kind: "method"
								key: JSStaticPropertyKey {
									value: JSIdentifier {
										name: "c"
										loc: SourceLocation experimental/class-properties/super/input.js 1:28-1:29 (c)
									}
									loc: SourceLocation experimental/class-properties/super/input.js 1:28-1:29
								}
								meta: JSClassPropertyMeta {
									abstract: false
									optional: false
									readonly: false
									static: false
									loc: SourceLocation experimental/class-properties/super/input.js 1:28-1:29
									start: Position 1:28
								}
								body: JSBlockStatement {
									body: []
									directives: []
									loc: SourceLocation experimental/class-properties/super/input.js 1:31-1:33
								}
								head: JSFunctionHead {
									async: false
									generator: false
									hasHoistedVars: false
									params: []
									loc: SourceLocation experimental/class-properties/super/input.js 1:29-1:31
								}
								loc: SourceLocation experimental/class-properties/super/input.js 1:28-1:33
							}
						]
						loc: SourceLocation experimental/class-properties/super/input.js 1:20-1:35
					}
					loc: SourceLocation experimental/class-properties/super/input.js 1:20-1:35
				}
				loc: SourceLocation experimental/class-properties/super/input.js 1:0-3:1
			}
			loc: SourceLocation experimental/class-properties/super/input.js 1:0-3:1
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<experimental/class-properties/super/input.js>
	loc: SourceLocation experimental/class-properties/super/input.js 1:0-4:0
}
```

### `diagnostics`

```

```