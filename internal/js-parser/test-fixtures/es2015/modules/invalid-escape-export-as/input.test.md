# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2015 > modules > invalid-escape-export-as`

### `ast`

```javascript
JSRoot {
	body: [
		JSExportLocalDeclaration {
			exportKind: "value"
			specifiers: [
				JSExportLocalSpecifier {
					exported: JSIdentifier {
						name: "X"
						loc: SourceLocation es2015/modules/invalid-escape-export-as/input.js 1:9-1:10 (X)
					}
					local: JSReferenceIdentifier {
						name: "X"
						loc: SourceLocation es2015/modules/invalid-escape-export-as/input.js 1:9-1:10 (X)
					}
					loc: SourceLocation es2015/modules/invalid-escape-export-as/input.js 1:9-1:10
				}
				JSExportLocalSpecifier {
					exported: JSIdentifier {
						name: "as"
						loc: SourceLocation es2015/modules/invalid-escape-export-as/input.js 1:11-1:18 (as)
					}
					local: JSReferenceIdentifier {
						name: "as"
						loc: SourceLocation es2015/modules/invalid-escape-export-as/input.js 1:11-1:18 (as)
					}
					loc: SourceLocation es2015/modules/invalid-escape-export-as/input.js 1:11-1:18
				}
				JSExportLocalSpecifier {
					exported: JSIdentifier {
						name: "Y"
						loc: SourceLocation es2015/modules/invalid-escape-export-as/input.js 1:19-1:20 (Y)
					}
					local: JSReferenceIdentifier {
						name: "Y"
						loc: SourceLocation es2015/modules/invalid-escape-export-as/input.js 1:19-1:20 (Y)
					}
					loc: SourceLocation es2015/modules/invalid-escape-export-as/input.js 1:19-1:20
				}
			]
			loc: SourceLocation es2015/modules/invalid-escape-export-as/input.js 1:0-1:22
		}
	]
	comments: []
	corrupt: false
	diagnostics: [
		{
			origins: [{entity: "ParserCore<js>"}]
			description: {
				advice: [log {category: "info", text: [RAW_MARKUP {value: "Expected character <emphasis>"}, ",", RAW_MARKUP {value: "</emphasis>"}]}]
				category: ["parse"]
				categoryValue: "js"
				message: [RAW_MARKUP {value: "Unexpected character <emphasis>"}, "\\", RAW_MARKUP {value: "</emphasis>"}]
			}
			location: {
				language: "js"
				path: UIDPath<es2015/modules/invalid-escape-export-as/input.js>
				end: Position 1:18
				start: Position 1:11
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "module"
	syntax: []
	path: UIDPath<es2015/modules/invalid-escape-export-as/input.js>
	loc: SourceLocation es2015/modules/invalid-escape-export-as/input.js 1:0-2:0
}
```

### `diagnostics`

```

 es2015/modules/invalid-escape-export-as/input.js:1:11 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ Unexpected character \

    export { X \u0061s Y }
               ^^^^^^^

  ℹ Expected character ,


```