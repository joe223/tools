# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > es2015-import-declaration > invalid-import-namespace-after-named`

### `ast`

```javascript
JSRoot {
	body: [
		JSImportDeclaration {
			namedSpecifiers: [
				JSImportSpecifier {
					imported: JSIdentifier {
						name: "bar"
						loc: SourceLocation esprima/es2015-import-declaration/invalid-import-namespace-after-named/input.js 1:8-1:11 (bar)
					}
					local: JSImportSpecifierLocal {
						name: JSBindingIdentifier {
							name: "bar"
							loc: SourceLocation esprima/es2015-import-declaration/invalid-import-namespace-after-named/input.js 1:8-1:11 (bar)
						}
						loc: SourceLocation esprima/es2015-import-declaration/invalid-import-namespace-after-named/input.js 1:8-1:11
					}
					loc: SourceLocation esprima/es2015-import-declaration/invalid-import-namespace-after-named/input.js 1:8-1:11
				}
			]
			source: JSStringLiteral {
				value: ""
				loc: SourceLocation esprima/es2015-import-declaration/invalid-import-namespace-after-named/input.js 1:0-1:12
			}
			loc: SourceLocation esprima/es2015-import-declaration/invalid-import-namespace-after-named/input.js 1:0-1:12
		}
		JSExpressionStatement {
			expression: JSBinaryExpression {
				operator: "*"
				left: JSReferenceIdentifier {
					name: "INVALID_PLACEHOLDER"
					loc: SourceLocation esprima/es2015-import-declaration/invalid-import-namespace-after-named/input.js 1:12-1:13
				}
				right: JSReferenceIdentifier {
					name: "as"
					loc: SourceLocation esprima/es2015-import-declaration/invalid-import-namespace-after-named/input.js 1:16-1:18 (as)
				}
				loc: SourceLocation esprima/es2015-import-declaration/invalid-import-namespace-after-named/input.js 1:12-1:18
			}
			loc: SourceLocation esprima/es2015-import-declaration/invalid-import-namespace-after-named/input.js 1:12-1:18
		}
		JSExpressionStatement {
			expression: JSReferenceIdentifier {
				name: "foo"
				loc: SourceLocation esprima/es2015-import-declaration/invalid-import-namespace-after-named/input.js 1:19-1:22 (foo)
			}
			loc: SourceLocation esprima/es2015-import-declaration/invalid-import-namespace-after-named/input.js 1:19-1:22
		}
		JSExpressionStatement {
			expression: JSReferenceIdentifier {
				name: "from"
				loc: SourceLocation esprima/es2015-import-declaration/invalid-import-namespace-after-named/input.js 1:23-1:27 (from)
			}
			loc: SourceLocation esprima/es2015-import-declaration/invalid-import-namespace-after-named/input.js 1:23-1:27
		}
		JSExpressionStatement {
			expression: JSStringLiteral {
				value: "foo"
				loc: SourceLocation esprima/es2015-import-declaration/invalid-import-namespace-after-named/input.js 1:28-1:33
			}
			loc: SourceLocation esprima/es2015-import-declaration/invalid-import-namespace-after-named/input.js 1:28-1:34
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
				message: [RAW_MARKUP {value: "Expected keyword <emphasis>"}, "from", RAW_MARKUP {value: "</emphasis>"}]
			}
			location: {
				language: "js"
				path: UIDPath<esprima/es2015-import-declaration/invalid-import-namespace-after-named/input.js>
				end: Position 1:12
				start: Position 1:12
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "module"
	syntax: []
	path: UIDPath<esprima/es2015-import-declaration/invalid-import-namespace-after-named/input.js>
	loc: SourceLocation esprima/es2015-import-declaration/invalid-import-namespace-after-named/input.js 1:0-2:0
}
```

### `diagnostics`

```

 esprima/es2015-import-declaration/invalid-import-namespace-after-named/input.js:1:12 parse(js) ━━━━

  ✖ Expected keyword from

    import {bar}, * as foo from "foo";
                ^


```