# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > invalid-syntax > migrated_0220`

### `ast`

```javascript
JSRoot {
	body: [
		JSFunctionDeclaration {
			id: JSBindingIdentifier {
				name: "hello"
				loc: SourceLocation esprima/invalid-syntax/migrated_0220/input.js 1:9-1:14 (hello)
			}
			body: JSBlockStatement {
				body: [
					JSExpressionStatement {
						expression: JSObjectExpression {
							properties: [
								JSObjectProperty {
									key: JSStaticPropertyKey {
										value: JSNumericLiteral {
											value: 17
											format: "octal"
											loc: SourceLocation esprima/invalid-syntax/migrated_0220/input.js 1:36-1:39
										}
										loc: SourceLocation esprima/invalid-syntax/migrated_0220/input.js 1:36-1:39
									}
									value: JSNumericLiteral {
										value: 42
										loc: SourceLocation esprima/invalid-syntax/migrated_0220/input.js 1:41-1:43
									}
									loc: SourceLocation esprima/invalid-syntax/migrated_0220/input.js 1:36-1:43
								}
							]
							loc: SourceLocation esprima/invalid-syntax/migrated_0220/input.js 1:34-1:45
						}
						loc: SourceLocation esprima/invalid-syntax/migrated_0220/input.js 1:33-1:47
					}
				]
				directives: [
					JSDirective {
						value: "use strict"
						loc: SourceLocation esprima/invalid-syntax/migrated_0220/input.js 1:19-1:32
					}
				]
				loc: SourceLocation esprima/invalid-syntax/migrated_0220/input.js 1:17-1:49
			}
			head: JSFunctionHead {
				async: false
				generator: false
				hasHoistedVars: false
				params: []
				loc: SourceLocation esprima/invalid-syntax/migrated_0220/input.js 1:14-1:16
			}
			loc: SourceLocation esprima/invalid-syntax/migrated_0220/input.js 1:0-1:49
		}
	]
	comments: []
	corrupt: false
	diagnostics: [
		{
			origins: [{entity: "ParserCore<js>"}]
			description: {
				advice: []
				category: ["parse"]
				categoryValue: "js"
				message: RAW_MARKUP {value: "Legacy octal literals are not allowed in strict mode"}
			}
			location: {
				language: "js"
				path: UIDPath<esprima/invalid-syntax/migrated_0220/input.js>
				end: Position 1:39
				start: Position 1:39
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<esprima/invalid-syntax/migrated_0220/input.js>
	loc: SourceLocation esprima/invalid-syntax/migrated_0220/input.js 1:0-2:0
}
```

### `diagnostics`

```

 esprima/invalid-syntax/migrated_0220/input.js:1:39 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Legacy octal literals are not allowed in strict mode

    function hello() { 'use strict'; ({ 021: 42 }); }
                                           ^


```