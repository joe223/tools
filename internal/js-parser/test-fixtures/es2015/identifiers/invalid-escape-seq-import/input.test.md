# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2015 > identifiers > invalid-escape-seq-import`

### `ast`

```javascript
JSRoot {
	body: [
		JSVariableDeclarationStatement {
			declaration: JSVariableDeclaration {
				kind: "var"
				declarations: [
					JSVariableDeclarator {
						id: JSBindingIdentifier {
							name: "import"
							loc: SourceLocation es2015/identifiers/invalid-escape-seq-import/input.js 1:4-1:40 (import)
						}
						init: JSNumericLiteral {
							value: 123
							loc: SourceLocation es2015/identifiers/invalid-escape-seq-import/input.js 1:43-1:46
						}
						loc: SourceLocation es2015/identifiers/invalid-escape-seq-import/input.js 1:4-1:46
					}
				]
				loc: SourceLocation es2015/identifiers/invalid-escape-seq-import/input.js 1:0-1:47
			}
			loc: SourceLocation es2015/identifiers/invalid-escape-seq-import/input.js 1:0-1:47
		}
		JSImportDeclaration {
			namedSpecifiers: []
			source: JSStringLiteral {
				value: "x"
				loc: SourceLocation es2015/identifiers/invalid-escape-seq-import/input.js 3:37-3:40
			}
			loc: SourceLocation es2015/identifiers/invalid-escape-seq-import/input.js 3:0-3:41
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
				message: [RAW_MARKUP {value: "Escape sequence in keyword <emphasis>"}, "import", RAW_MARKUP {value: "</emphasis>"}]
			}
			location: {
				language: "js"
				path: UIDPath<es2015/identifiers/invalid-escape-seq-import/input.js>
				end: Position 1:34
				start: Position 1:34
			}
		}
	]
	directives: []
	hasHoistedVars: true
	sourceType: "module"
	syntax: []
	path: UIDPath<es2015/identifiers/invalid-escape-seq-import/input.js>
	loc: SourceLocation es2015/identifiers/invalid-escape-seq-import/input.js 1:0-4:0
}
```

### `diagnostics`

```

 es2015/identifiers/invalid-escape-seq-import/input.js:1:34 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Escape sequence in keyword import

  > 1 │ var \u{69}\u{6d}\u{70}\u{6f}\u{72}\u{74} = 123;
      │                                   ^
    2 │
    3 │ \u{69}\u{6d}\u{70}\u{6f}\u{72}\u{74} "x";


```