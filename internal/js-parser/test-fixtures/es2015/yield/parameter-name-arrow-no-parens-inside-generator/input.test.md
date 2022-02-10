# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2015 > yield > parameter-name-arrow-no-parens-inside-generator`

### `ast`

```javascript
JSRoot {
	body: [
		JSFunctionDeclaration {
			id: JSBindingIdentifier {
				name: "fn"
				loc: SourceLocation es2015/yield/parameter-name-arrow-no-parens-inside-generator/input.js 1:10-1:12 (fn)
			}
			body: JSBlockStatement {
				body: [
					JSExpressionStatement {
						expression: JSYieldExpression {
							delegate: false
							loc: SourceLocation es2015/yield/parameter-name-arrow-no-parens-inside-generator/input.js 2:2-2:7
						}
						loc: SourceLocation es2015/yield/parameter-name-arrow-no-parens-inside-generator/input.js 2:2-2:7
					}
					JSExpressionStatement {
						expression: JSReferenceIdentifier {
							name: "INVALID_PLACEHOLDER"
							loc: SourceLocation es2015/yield/parameter-name-arrow-no-parens-inside-generator/input.js 2:8-2:10
						}
						loc: SourceLocation es2015/yield/parameter-name-arrow-no-parens-inside-generator/input.js 2:8-2:10
					}
					JSBlockStatement {
						body: []
						directives: []
						loc: SourceLocation es2015/yield/parameter-name-arrow-no-parens-inside-generator/input.js 2:11-2:13
					}
					JSEmptyStatement {
						loc: SourceLocation es2015/yield/parameter-name-arrow-no-parens-inside-generator/input.js 2:13-2:14
					}
				]
				directives: []
				loc: SourceLocation es2015/yield/parameter-name-arrow-no-parens-inside-generator/input.js 1:15-3:1
			}
			head: JSFunctionHead {
				async: false
				generator: true
				hasHoistedVars: false
				params: []
				loc: SourceLocation es2015/yield/parameter-name-arrow-no-parens-inside-generator/input.js 1:12-1:14
			}
			loc: SourceLocation es2015/yield/parameter-name-arrow-no-parens-inside-generator/input.js 1:0-3:1
		}
	]
	comments: []
	corrupt: true
	diagnostics: [
		{
			origins: [{entity: "ParserCore<js>"}]
			description: {
				advice: []
				category: ["parse"]
				categoryValue: "js"
				message: RAW_MARKUP {value: "Expected a semicolon or a line terminator"}
			}
			location: {
				language: "js"
				path: UIDPath<es2015/yield/parameter-name-arrow-no-parens-inside-generator/input.js>
				end: Position 2:7
				start: Position 2:8
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<es2015/yield/parameter-name-arrow-no-parens-inside-generator/input.js>
	loc: SourceLocation es2015/yield/parameter-name-arrow-no-parens-inside-generator/input.js 1:0-3:1
}
```

### `diagnostics`

```

 es2015/yield/parameter-name-arrow-no-parens-inside-generator/input.js:2:8 parse(js) ━━━━━━━━━━━━━━━

  ✖ Expected a semicolon or a line terminator

    1 │ function* fn() {
  > 2 │   yield => {};
      │         ^
    3 │ }


```