# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `es2015 > uncategorised > 165`

### `ast`

```javascript
JSRoot {
	body: [
		JSFunctionDeclaration {
			id: JSBindingIdentifier {
				name: "x"
				loc: SourceLocation es2015/uncategorised/165/input.js 1:9-1:10 (x)
			}
			body: JSBlockStatement {
				body: []
				directives: []
				loc: SourceLocation es2015/uncategorised/165/input.js 1:20-1:22
			}
			head: JSFunctionHead {
				async: false
				generator: false
				hasHoistedVars: false
				params: [
					JSBindingObjectPattern {
						properties: [
							JSBindingObjectPatternProperty {
								key: JSStaticPropertyKey {
									value: JSIdentifier {
										name: "a"
										loc: SourceLocation es2015/uncategorised/165/input.js 1:13-1:14 (a)
									}
									loc: SourceLocation es2015/uncategorised/165/input.js 1:13-1:14
								}
								value: JSBindingIdentifier {
									name: "a"
									loc: SourceLocation es2015/uncategorised/165/input.js 1:13-1:14 (a)
								}
								loc: SourceLocation es2015/uncategorised/165/input.js 1:13-1:14
							}
							JSBindingObjectPatternProperty {
								key: JSStaticPropertyKey {
									value: JSIdentifier {
										name: "b"
										loc: SourceLocation es2015/uncategorised/165/input.js 1:16-1:17 (b)
									}
									loc: SourceLocation es2015/uncategorised/165/input.js 1:16-1:17
								}
								value: JSBindingIdentifier {
									name: "b"
									loc: SourceLocation es2015/uncategorised/165/input.js 1:16-1:17 (b)
								}
								loc: SourceLocation es2015/uncategorised/165/input.js 1:16-1:17
							}
						]
						meta: JSPatternMeta {
							loc: SourceLocation es2015/uncategorised/165/input.js 1:11-1:19
						}
						loc: SourceLocation es2015/uncategorised/165/input.js 1:11-1:19
					}
				]
				loc: SourceLocation es2015/uncategorised/165/input.js 1:10-1:20
			}
			loc: SourceLocation es2015/uncategorised/165/input.js 1:0-1:22
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	directives: []
	hasHoistedVars: false
	sourceType: "script"
	syntax: []
	path: UIDPath<es2015/uncategorised/165/input.js>
	loc: SourceLocation es2015/uncategorised/165/input.js 1:0-1:22
}
```

### `diagnostics`

```

```