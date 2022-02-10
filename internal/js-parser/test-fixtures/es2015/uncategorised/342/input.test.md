# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2015 > uncategorised > 342`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSAssignmentExpression {
				operator: "="
				left: JSAssignmentIdentifier {
					name: "x"
					loc: SourceLocation es2015/uncategorised/342/input.js 1:0-1:1 (x)
				}
				right: JSObjectExpression {
					properties: [
						JSObjectMethod {
							kind: "set"
							key: JSStaticPropertyKey {
								value: JSIdentifier {
									name: "method"
									loc: SourceLocation es2015/uncategorised/342/input.js 1:10-1:16 (method)
								}
								loc: SourceLocation es2015/uncategorised/342/input.js 1:10-1:16
							}
							body: JSBlockStatement {
								body: [
									JSExpressionStatement {
										expression: JSAssignmentExpression {
											operator: "="
											left: JSAssignmentIdentifier {
												name: "v"
												loc: SourceLocation es2015/uncategorised/342/input.js 1:22-1:23 (v)
											}
											right: JSReferenceIdentifier {
												name: "val"
												loc: SourceLocation es2015/uncategorised/342/input.js 1:26-1:29 (val)
											}
											loc: SourceLocation es2015/uncategorised/342/input.js 1:22-1:29
										}
										loc: SourceLocation es2015/uncategorised/342/input.js 1:22-1:29
									}
								]
								directives: []
								loc: SourceLocation es2015/uncategorised/342/input.js 1:22-1:31
							}
							head: JSFunctionHead {
								async: false
								generator: false
								hasHoistedVars: false
								params: [
									JSBindingIdentifier {
										name: "val"
										meta: JSPatternMeta {
											loc: SourceLocation es2015/uncategorised/342/input.js 1:17-1:20
										}
										loc: SourceLocation es2015/uncategorised/342/input.js 1:17-1:20 (val)
									}
								]
								loc: SourceLocation es2015/uncategorised/342/input.js 1:16-1:21
							}
							loc: SourceLocation es2015/uncategorised/342/input.js 1:6-1:31
						}
					]
					loc: SourceLocation es2015/uncategorised/342/input.js 1:4-1:31
				}
				loc: SourceLocation es2015/uncategorised/342/input.js 1:0-1:31
			}
			loc: SourceLocation es2015/uncategorised/342/input.js 1:0-1:31
		}
	]
	comments: []
	corrupt: false
	diagnostics: [
		{
			origins: [{entity: "ParserCore<js>"}]
			description: {
				advice: [
					log {
						category: "info"
						text: [RAW_MARKUP {value: "Expected the opening "}, "block", RAW_MARKUP {value: " character <emphasis>"}, "{", RAW_MARKUP {value: "</emphasis>"}]
					}
				]
				category: ["parse"]
				categoryValue: "js"
				message: [RAW_MARKUP {value: "Unexpected character <emphasis>"}, "v", RAW_MARKUP {value: "</emphasis>"}]
			}
			location: {
				language: "js"
				path: UIDPath<es2015/uncategorised/342/input.js>
				end: Position 1:21
				start: Position 1:22
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<es2015/uncategorised/342/input.js>
	loc: SourceLocation es2015/uncategorised/342/input.js 1:0-1:31
}
```

### `diagnostics`

```

 es2015/uncategorised/342/input.js:1:22 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Unexpected character v

    x = { set method(val) v = val }
                          ^

  ℹ Expected the opening block character {


```