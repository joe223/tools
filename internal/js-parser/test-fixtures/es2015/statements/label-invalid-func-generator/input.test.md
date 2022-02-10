# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2015 > statements > label-invalid-func-generator`

### `ast`

```javascript
JSRoot {
	body: [
		JSLabeledStatement {
			body: JSFunctionDeclaration {
				id: JSBindingIdentifier {
					name: "bar"
					loc: SourceLocation es2015/statements/label-invalid-func-generator/input.js 1:15-1:18 (bar)
				}
				body: JSBlockStatement {
					body: []
					directives: []
					loc: SourceLocation es2015/statements/label-invalid-func-generator/input.js 1:21-1:23
				}
				head: JSFunctionHead {
					async: false
					generator: true
					hasHoistedVars: false
					params: []
					loc: SourceLocation es2015/statements/label-invalid-func-generator/input.js 1:18-1:20
				}
				loc: SourceLocation es2015/statements/label-invalid-func-generator/input.js 1:5-1:23
			}
			label: JSIdentifier {
				name: "foo"
				loc: SourceLocation es2015/statements/label-invalid-func-generator/input.js 1:0-1:3 (foo)
			}
			loc: SourceLocation es2015/statements/label-invalid-func-generator/input.js 1:0-1:23
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
				message: RAW_MARKUP {value: "Generators can only be declared at the top level or inside a block"}
			}
			location: {
				language: "js"
				path: UIDPath<es2015/statements/label-invalid-func-generator/input.js>
				end: Position 1:23
				start: Position 1:5
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<es2015/statements/label-invalid-func-generator/input.js>
	loc: SourceLocation es2015/statements/label-invalid-func-generator/input.js 1:0-2:0
}
```

### `diagnostics`

```

 es2015/statements/label-invalid-func-generator/input.js:1:5 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Generators can only be declared at the top level or inside a block

    foo: function* bar() {}
         ^^^^^^^^^^^^^^^^^^


```