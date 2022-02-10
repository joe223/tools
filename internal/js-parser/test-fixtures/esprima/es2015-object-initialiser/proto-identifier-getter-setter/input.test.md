# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > es2015-object-initialiser > proto-identifier-getter-setter`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSObjectExpression {
				properties: [
					JSObjectProperty {
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "__proto__"
								loc: SourceLocation esprima/es2015-object-initialiser/proto-identifier-getter-setter/input.js 1:3-1:12 (__proto__)
							}
							loc: SourceLocation esprima/es2015-object-initialiser/proto-identifier-getter-setter/input.js 1:3-1:12
						}
						value: JSNullLiteral {
							loc: SourceLocation esprima/es2015-object-initialiser/proto-identifier-getter-setter/input.js 1:14-1:18
						}
						loc: SourceLocation esprima/es2015-object-initialiser/proto-identifier-getter-setter/input.js 1:3-1:18
					}
					JSObjectMethod {
						kind: "get"
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "__proto__"
								loc: SourceLocation esprima/es2015-object-initialiser/proto-identifier-getter-setter/input.js 1:24-1:33 (__proto__)
							}
							loc: SourceLocation esprima/es2015-object-initialiser/proto-identifier-getter-setter/input.js 1:24-1:33
						}
						body: JSBlockStatement {
							body: []
							directives: []
							loc: SourceLocation esprima/es2015-object-initialiser/proto-identifier-getter-setter/input.js 1:35-1:37
						}
						head: JSFunctionHead {
							async: false
							generator: false
							hasHoistedVars: false
							params: []
							loc: SourceLocation esprima/es2015-object-initialiser/proto-identifier-getter-setter/input.js 1:33-1:35
						}
						loc: SourceLocation esprima/es2015-object-initialiser/proto-identifier-getter-setter/input.js 1:20-1:37
					}
					JSObjectMethod {
						kind: "set"
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "__proto__"
								loc: SourceLocation esprima/es2015-object-initialiser/proto-identifier-getter-setter/input.js 1:43-1:52 (__proto__)
							}
							loc: SourceLocation esprima/es2015-object-initialiser/proto-identifier-getter-setter/input.js 1:43-1:52
						}
						body: JSBlockStatement {
							body: []
							directives: []
							loc: SourceLocation esprima/es2015-object-initialiser/proto-identifier-getter-setter/input.js 1:55-1:57
						}
						head: JSFunctionHead {
							async: false
							generator: false
							hasHoistedVars: false
							params: [
								JSBindingIdentifier {
									name: "x"
									meta: JSPatternMeta {
										loc: SourceLocation esprima/es2015-object-initialiser/proto-identifier-getter-setter/input.js 1:53-1:54
									}
									loc: SourceLocation esprima/es2015-object-initialiser/proto-identifier-getter-setter/input.js 1:53-1:54 (x)
								}
							]
							loc: SourceLocation esprima/es2015-object-initialiser/proto-identifier-getter-setter/input.js 1:52-1:55
						}
						loc: SourceLocation esprima/es2015-object-initialiser/proto-identifier-getter-setter/input.js 1:39-1:57
					}
				]
				loc: SourceLocation esprima/es2015-object-initialiser/proto-identifier-getter-setter/input.js 1:1-1:59
			}
			loc: SourceLocation esprima/es2015-object-initialiser/proto-identifier-getter-setter/input.js 1:0-1:60
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/es2015-object-initialiser/proto-identifier-getter-setter/input.js>
	loc: SourceLocation esprima/es2015-object-initialiser/proto-identifier-getter-setter/input.js 1:0-2:0
}
```

### `diagnostics`

```

```