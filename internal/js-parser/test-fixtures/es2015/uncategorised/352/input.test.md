# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2015 > uncategorised > 352`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSObjectExpression {
				properties: [
					JSObjectMethod {
						kind: "get"
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "__proto__"
								loc: SourceLocation es2015/uncategorised/352/input.js 1:7-1:16 (__proto__)
							}
							loc: SourceLocation es2015/uncategorised/352/input.js 1:7-1:16
						}
						body: JSBlockStatement {
							body: [
								JSReturnStatement {
									argument: JSNumericLiteral {
										value: 1
										loc: SourceLocation es2015/uncategorised/352/input.js 1:28-1:29
									}
									loc: SourceLocation es2015/uncategorised/352/input.js 1:21-1:29
								}
							]
							directives: []
							loc: SourceLocation es2015/uncategorised/352/input.js 1:19-1:31
						}
						head: JSFunctionHead {
							async: false
							generator: false
							hasHoistedVars: false
							params: []
							loc: SourceLocation es2015/uncategorised/352/input.js 1:16-1:18
						}
						loc: SourceLocation es2015/uncategorised/352/input.js 1:3-1:31
					}
					JSObjectProperty {
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "__proto__"
								loc: SourceLocation es2015/uncategorised/352/input.js 1:33-1:42 (__proto__)
							}
							loc: SourceLocation es2015/uncategorised/352/input.js 1:33-1:42
						}
						value: JSNumericLiteral {
							value: 2
							loc: SourceLocation es2015/uncategorised/352/input.js 1:44-1:45
						}
						loc: SourceLocation es2015/uncategorised/352/input.js 1:33-1:45
					}
				]
				loc: SourceLocation es2015/uncategorised/352/input.js 1:1-1:47
			}
			loc: SourceLocation es2015/uncategorised/352/input.js 1:0-1:48
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<es2015/uncategorised/352/input.js>
	loc: SourceLocation es2015/uncategorised/352/input.js 1:0-1:48
}
```

### `diagnostics`

```

```