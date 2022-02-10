# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `experimental > class-private-names-duplicated > static-get-static-field`

### `ast`

```javascript
JSRoot {
	body: [
		JSClassDeclaration {
			id: JSBindingIdentifier {
				name: "A"
				loc: SourceLocation experimental/class-private-names-duplicated/static-get-static-field/input.js 1:6-1:7 (A)
			}
			meta: JSClassHead {
				body: [
					JSClassPrivateMethod {
						kind: "get"
						key: JSPrivateName {
							id: JSIdentifier {
								name: "x"
								loc: SourceLocation experimental/class-private-names-duplicated/static-get-static-field/input.js 2:14-2:15 (x)
							}
							loc: SourceLocation experimental/class-private-names-duplicated/static-get-static-field/input.js 2:13-2:15
						}
						meta: JSClassPropertyMeta {
							abstract: false
							optional: false
							readonly: false
							static: true
							loc: SourceLocation experimental/class-private-names-duplicated/static-get-static-field/input.js 2:2-2:15
							start: Position 2:2
						}
						body: JSBlockStatement {
							body: []
							directives: []
							loc: SourceLocation experimental/class-private-names-duplicated/static-get-static-field/input.js 2:18-2:20
						}
						head: JSFunctionHead {
							async: false
							generator: false
							hasHoistedVars: false
							params: []
							loc: SourceLocation experimental/class-private-names-duplicated/static-get-static-field/input.js 2:15-2:17
						}
						loc: SourceLocation experimental/class-private-names-duplicated/static-get-static-field/input.js 2:2-2:20
					}
					JSClassPrivateProperty {
						key: JSPrivateName {
							id: JSIdentifier {
								name: "x"
								loc: SourceLocation experimental/class-private-names-duplicated/static-get-static-field/input.js 3:10-3:11 (x)
							}
							loc: SourceLocation experimental/class-private-names-duplicated/static-get-static-field/input.js 3:9-3:11
						}
						meta: JSClassPropertyMeta {
							abstract: false
							optional: false
							readonly: false
							static: true
							loc: SourceLocation experimental/class-private-names-duplicated/static-get-static-field/input.js 3:2-3:11
							start: Position 3:2
						}
						value: JSNumericLiteral {
							value: 0
							loc: SourceLocation experimental/class-private-names-duplicated/static-get-static-field/input.js 3:14-3:15
						}
						loc: SourceLocation experimental/class-private-names-duplicated/static-get-static-field/input.js 3:2-3:16
					}
				]
				loc: SourceLocation experimental/class-private-names-duplicated/static-get-static-field/input.js 1:0-4:1
			}
			loc: SourceLocation experimental/class-private-names-duplicated/static-get-static-field/input.js 1:0-4:1
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<experimental/class-private-names-duplicated/static-get-static-field/input.js>
	loc: SourceLocation experimental/class-private-names-duplicated/static-get-static-field/input.js 1:0-4:1
}
```

### `diagnostics`

```

```