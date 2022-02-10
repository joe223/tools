# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2015 > uncategorised > 289`

### `ast`

```javascript
JSRoot {
	body: [
		JSExpressionStatement {
			expression: JSObjectExpression {
				properties: [
					JSObjectMethod {
						kind: "method"
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "t"
								loc: SourceLocation es2015/uncategorised/289/input.js 1:3-1:4 (t)
							}
							loc: SourceLocation es2015/uncategorised/289/input.js 1:3-1:4
						}
						body: JSBlockStatement {
							body: []
							directives: [
								JSDirective {
									value: "use strict"
									loc: SourceLocation es2015/uncategorised/289/input.js 1:13-1:26
								}
							]
							loc: SourceLocation es2015/uncategorised/289/input.js 1:11-1:28
						}
						head: JSFunctionHead {
							async: false
							generator: false
							hasHoistedVars: false
							params: [
								JSBindingIdentifier {
									name: "eval"
									meta: JSPatternMeta {
										loc: SourceLocation es2015/uncategorised/289/input.js 1:5-1:9
									}
									loc: SourceLocation es2015/uncategorised/289/input.js 1:5-1:9 (eval)
								}
							]
							loc: SourceLocation es2015/uncategorised/289/input.js 1:4-1:10
						}
						loc: SourceLocation es2015/uncategorised/289/input.js 1:3-1:28
					}
				]
				loc: SourceLocation es2015/uncategorised/289/input.js 1:1-1:30
			}
			loc: SourceLocation es2015/uncategorised/289/input.js 1:0-1:32
		}
	]
	comments: []
	corrupt: false
	diagnostics: [
		{
			origins: [{entity: "ParserCore<js>"}]
			description: {advice: [], category: ["parse"], categoryValue: "js", message: ["eval", RAW_MARKUP {value: " is a reserved word"}]}
			location: {
				language: "js"
				path: UIDPath<es2015/uncategorised/289/input.js>
				end: Position 1:9
				start: Position 1:5
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<es2015/uncategorised/289/input.js>
	loc: SourceLocation es2015/uncategorised/289/input.js 1:0-1:32
}
```

### `diagnostics`

```

 es2015/uncategorised/289/input.js:1:5 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ eval is a reserved word

    ({ t(eval) { "use strict"; } });
         ^^^^


```