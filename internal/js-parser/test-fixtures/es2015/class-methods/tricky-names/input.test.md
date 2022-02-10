# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2015 > class-methods > tricky-names`

### `ast`

```javascript
JSRoot {
	body: [
		JSClassDeclaration {
			id: JSBindingIdentifier {
				name: "A"
				loc: SourceLocation es2015/class-methods/tricky-names/input.js 1:6-1:7 (A)
			}
			meta: JSClassHead {
				body: [
					JSClassMethod {
						kind: "method"
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "get"
								loc: SourceLocation es2015/class-methods/tricky-names/input.js 2:2-2:5 (get)
							}
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 2:2-2:5
						}
						meta: JSClassPropertyMeta {
							abstract: false
							optional: false
							readonly: false
							static: false
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 2:2-2:5
							start: Position 2:2
						}
						body: JSBlockStatement {
							body: []
							directives: []
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 3:5-3:7
						}
						head: JSFunctionHead {
							async: false
							generator: false
							hasHoistedVars: false
							params: []
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 3:2-3:4
						}
						loc: SourceLocation es2015/class-methods/tricky-names/input.js 2:2-3:7
					}
					JSClassMethod {
						kind: "method"
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "set"
								loc: SourceLocation es2015/class-methods/tricky-names/input.js 5:2-5:5 (set)
							}
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 5:2-5:5
						}
						meta: JSClassPropertyMeta {
							abstract: false
							optional: false
							readonly: false
							static: false
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 5:2-5:5
							start: Position 5:2
						}
						body: JSBlockStatement {
							body: []
							directives: []
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 6:5-6:7
						}
						head: JSFunctionHead {
							async: false
							generator: false
							hasHoistedVars: false
							params: []
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 6:2-6:4
						}
						loc: SourceLocation es2015/class-methods/tricky-names/input.js 5:2-6:7
					}
					JSClassMethod {
						kind: "method"
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "static"
								loc: SourceLocation es2015/class-methods/tricky-names/input.js 8:2-8:8 (static)
							}
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 8:2-8:8 (static)
						}
						meta: JSClassPropertyMeta {
							abstract: false
							optional: false
							readonly: false
							static: false
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 8:2-8:8
						}
						body: JSBlockStatement {
							body: []
							directives: []
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 9:5-9:7
						}
						head: JSFunctionHead {
							async: false
							generator: false
							hasHoistedVars: false
							params: []
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 9:2-9:4
						}
						loc: SourceLocation es2015/class-methods/tricky-names/input.js 8:2-9:7
					}
					JSClassMethod {
						kind: "method"
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "async"
								loc: SourceLocation es2015/class-methods/tricky-names/input.js 11:2-11:7 (async)
							}
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 11:2-11:7
						}
						meta: JSClassPropertyMeta {
							abstract: false
							optional: false
							readonly: false
							static: false
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 11:2-11:7
							start: Position 11:2
						}
						body: JSBlockStatement {
							body: []
							directives: []
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 12:5-12:7
						}
						head: JSFunctionHead {
							async: false
							generator: false
							hasHoistedVars: false
							params: []
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 12:2-12:4
						}
						loc: SourceLocation es2015/class-methods/tricky-names/input.js 11:2-12:7
					}
					JSClassMethod {
						kind: "method"
						key: JSStaticPropertyKey {
							value: JSStringLiteral {
								value: "get"
								loc: SourceLocation es2015/class-methods/tricky-names/input.js 15:2-15:7
							}
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 15:2-15:7
						}
						meta: JSClassPropertyMeta {
							abstract: false
							optional: false
							readonly: false
							static: false
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 15:2-15:7
							start: Position 15:2
						}
						body: JSBlockStatement {
							body: []
							directives: []
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 16:5-16:7
						}
						head: JSFunctionHead {
							async: false
							generator: false
							hasHoistedVars: false
							params: []
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 16:2-16:4
						}
						loc: SourceLocation es2015/class-methods/tricky-names/input.js 15:2-16:7
					}
					JSClassMethod {
						kind: "method"
						key: JSStaticPropertyKey {
							value: JSStringLiteral {
								value: "set"
								loc: SourceLocation es2015/class-methods/tricky-names/input.js 18:2-18:7
							}
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 18:2-18:7
						}
						meta: JSClassPropertyMeta {
							abstract: false
							optional: false
							readonly: false
							static: false
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 18:2-18:7
							start: Position 18:2
						}
						body: JSBlockStatement {
							body: []
							directives: []
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 19:5-19:7
						}
						head: JSFunctionHead {
							async: false
							generator: false
							hasHoistedVars: false
							params: []
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 19:2-19:4
						}
						loc: SourceLocation es2015/class-methods/tricky-names/input.js 18:2-19:7
					}
					JSClassMethod {
						kind: "method"
						key: JSStaticPropertyKey {
							value: JSStringLiteral {
								value: "async"
								loc: SourceLocation es2015/class-methods/tricky-names/input.js 21:2-21:9
							}
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 21:2-21:9
						}
						meta: JSClassPropertyMeta {
							abstract: false
							optional: false
							readonly: false
							static: false
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 21:2-21:9
							start: Position 21:2
						}
						body: JSBlockStatement {
							body: []
							directives: []
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 22:5-22:7
						}
						head: JSFunctionHead {
							async: false
							generator: false
							hasHoistedVars: false
							params: []
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 22:2-22:4
						}
						loc: SourceLocation es2015/class-methods/tricky-names/input.js 21:2-22:7
					}
					JSClassMethod {
						kind: "method"
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "get"
								loc: SourceLocation es2015/class-methods/tricky-names/input.js 26:2-26:5 (get)
							}
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 26:2-26:5
						}
						meta: JSClassPropertyMeta {
							abstract: false
							optional: false
							readonly: false
							static: true
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 25:2-26:5
							start: Position 25:2
						}
						body: JSBlockStatement {
							body: []
							directives: []
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 27:5-27:7
						}
						head: JSFunctionHead {
							async: false
							generator: false
							hasHoistedVars: false
							params: []
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 27:2-27:4
						}
						loc: SourceLocation es2015/class-methods/tricky-names/input.js 25:2-27:7
					}
					JSClassMethod {
						kind: "method"
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "set"
								loc: SourceLocation es2015/class-methods/tricky-names/input.js 30:2-30:5 (set)
							}
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 30:2-30:5
						}
						meta: JSClassPropertyMeta {
							abstract: false
							optional: false
							readonly: false
							static: true
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 29:2-30:5
							start: Position 29:2
						}
						body: JSBlockStatement {
							body: []
							directives: []
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 31:5-31:7
						}
						head: JSFunctionHead {
							async: false
							generator: false
							hasHoistedVars: false
							params: []
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 31:2-31:4
						}
						loc: SourceLocation es2015/class-methods/tricky-names/input.js 29:2-31:7
					}
					JSClassMethod {
						kind: "method"
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "static"
								loc: SourceLocation es2015/class-methods/tricky-names/input.js 34:2-34:8 (static)
							}
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 34:2-34:8
						}
						meta: JSClassPropertyMeta {
							abstract: false
							optional: false
							readonly: false
							static: true
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 33:2-34:8
							start: Position 33:2
						}
						body: JSBlockStatement {
							body: []
							directives: []
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 35:5-35:7
						}
						head: JSFunctionHead {
							async: false
							generator: false
							hasHoistedVars: false
							params: []
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 35:2-35:4
						}
						loc: SourceLocation es2015/class-methods/tricky-names/input.js 33:2-35:7
					}
					JSClassMethod {
						kind: "method"
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "async"
								loc: SourceLocation es2015/class-methods/tricky-names/input.js 38:2-38:7 (async)
							}
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 38:2-38:7
						}
						meta: JSClassPropertyMeta {
							abstract: false
							optional: false
							readonly: false
							static: true
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 37:2-38:7
							start: Position 37:2
						}
						body: JSBlockStatement {
							body: []
							directives: []
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 39:5-39:7
						}
						head: JSFunctionHead {
							async: false
							generator: false
							hasHoistedVars: false
							params: []
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 39:2-39:4
						}
						loc: SourceLocation es2015/class-methods/tricky-names/input.js 37:2-39:7
					}
					JSClassMethod {
						kind: "method"
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "a"
								loc: SourceLocation es2015/class-methods/tricky-names/input.js 42:2-42:3 (a)
							}
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 42:2-42:3
						}
						meta: JSClassPropertyMeta {
							abstract: false
							optional: false
							readonly: false
							static: true
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 41:2-42:3
							start: Position 41:2
						}
						body: JSBlockStatement {
							body: []
							directives: []
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 43:5-43:7
						}
						head: JSFunctionHead {
							async: false
							generator: false
							hasHoistedVars: false
							params: []
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 43:2-43:4
						}
						loc: SourceLocation es2015/class-methods/tricky-names/input.js 41:2-43:7
					}
					JSClassMethod {
						kind: "get"
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "async"
								loc: SourceLocation es2015/class-methods/tricky-names/input.js 47:2-47:7 (async)
							}
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 47:2-47:7
						}
						meta: JSClassPropertyMeta {
							abstract: false
							optional: false
							readonly: false
							static: false
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 46:2-47:7
							start: Position 46:2
						}
						body: JSBlockStatement {
							body: []
							directives: []
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 48:5-48:7
						}
						head: JSFunctionHead {
							async: false
							generator: false
							hasHoistedVars: false
							params: []
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 48:2-48:4
						}
						loc: SourceLocation es2015/class-methods/tricky-names/input.js 46:2-48:7
					}
					JSClassMethod {
						kind: "get"
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "static"
								loc: SourceLocation es2015/class-methods/tricky-names/input.js 53:2-53:8 (static)
							}
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 53:2-53:8
						}
						meta: JSClassPropertyMeta {
							abstract: false
							optional: false
							readonly: false
							static: true
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 51:2-53:8
							start: Position 51:2
						}
						body: JSBlockStatement {
							body: []
							directives: []
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 54:5-54:7
						}
						head: JSFunctionHead {
							async: false
							generator: false
							hasHoistedVars: false
							params: []
							loc: SourceLocation es2015/class-methods/tricky-names/input.js 54:2-54:4
						}
						loc: SourceLocation es2015/class-methods/tricky-names/input.js 51:2-54:7
					}
				]
				loc: SourceLocation es2015/class-methods/tricky-names/input.js 1:0-55:1
			}
			loc: SourceLocation es2015/class-methods/tricky-names/input.js 1:0-55:1
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<es2015/class-methods/tricky-names/input.js>
	loc: SourceLocation es2015/class-methods/tricky-names/input.js 1:0-56:0
}
```

### `diagnostics`

```

```