# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `experimental > class-private-names-duplicated > instance-field-instance-set`

### `ast`

```javascript
JSRoot {
	body: [
		JSClassDeclaration {
			id: JSBindingIdentifier {
				name: "A"
				loc: SourceLocation experimental/class-private-names-duplicated/instance-field-instance-set/input.js 1:6-1:7 (A)
			}
			meta: JSClassHead {
				body: [
					JSClassPrivateProperty {
						key: JSPrivateName {
							id: JSIdentifier {
								name: "x"
								loc: SourceLocation experimental/class-private-names-duplicated/instance-field-instance-set/input.js 2:3-2:4 (x)
							}
							loc: SourceLocation experimental/class-private-names-duplicated/instance-field-instance-set/input.js 2:2-2:4
						}
						meta: JSClassPropertyMeta {
							abstract: false
							optional: false
							readonly: false
							static: false
							loc: SourceLocation experimental/class-private-names-duplicated/instance-field-instance-set/input.js 2:2-2:4
							start: Position 2:2
						}
						value: JSNumericLiteral {
							value: 0
							loc: SourceLocation experimental/class-private-names-duplicated/instance-field-instance-set/input.js 2:7-2:8
						}
						loc: SourceLocation experimental/class-private-names-duplicated/instance-field-instance-set/input.js 2:2-2:9
					}
					JSClassPrivateMethod {
						kind: "set"
						key: JSPrivateName {
							id: JSIdentifier {
								name: "x"
								loc: SourceLocation experimental/class-private-names-duplicated/instance-field-instance-set/input.js 3:7-3:8 (x)
							}
							loc: SourceLocation experimental/class-private-names-duplicated/instance-field-instance-set/input.js 3:6-3:8
						}
						meta: JSClassPropertyMeta {
							abstract: false
							optional: false
							readonly: false
							static: false
							loc: SourceLocation experimental/class-private-names-duplicated/instance-field-instance-set/input.js 3:2-3:8
							start: Position 3:2
						}
						body: JSBlockStatement {
							body: []
							directives: []
							loc: SourceLocation experimental/class-private-names-duplicated/instance-field-instance-set/input.js 3:12-3:14
						}
						head: JSFunctionHead {
							async: false
							generator: false
							hasHoistedVars: false
							params: [
								JSBindingIdentifier {
									name: "_"
									meta: JSPatternMeta {
										loc: SourceLocation experimental/class-private-names-duplicated/instance-field-instance-set/input.js 3:9-3:10
									}
									loc: SourceLocation experimental/class-private-names-duplicated/instance-field-instance-set/input.js 3:9-3:10 (_)
								}
							]
							loc: SourceLocation experimental/class-private-names-duplicated/instance-field-instance-set/input.js 3:8-3:11
						}
						loc: SourceLocation experimental/class-private-names-duplicated/instance-field-instance-set/input.js 3:2-3:14
					}
				]
				loc: SourceLocation experimental/class-private-names-duplicated/instance-field-instance-set/input.js 1:0-4:1
			}
			loc: SourceLocation experimental/class-private-names-duplicated/instance-field-instance-set/input.js 1:0-4:1
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<experimental/class-private-names-duplicated/instance-field-instance-set/input.js>
	loc: SourceLocation experimental/class-private-names-duplicated/instance-field-instance-set/input.js 1:0-4:1
}
```

### `diagnostics`

```

```