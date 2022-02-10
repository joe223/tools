# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/markdown-parser/index.test.ts --update-snapshots` to update.

## `lists`

### `ast`

```javascript
MarkdownRoot {
	body: [
		MarkdownListBlock {
			children: [
				MarkdownListItem {
					value: "-"
					children: [
						MarkdownParagraph {
							children: [
								MarkdownText {
									value: "foo"
									loc: SourceLocation lists/input.md 1:2-1:5
								}
							]
							loc: SourceLocation lists/input.md 1:2-1:5
						}
					]
					loc: SourceLocation lists/input.md 1:2-1:5
				}
			]
			ordered: false
			loc: SourceLocation lists/input.md 1:0-1:5
		}
		MarkdownListBlock {
			children: [
				MarkdownListItem {
					value: "-"
					children: [
						MarkdownParagraph {
							children: [
								MarkdownText {
									value: "bar"
									loc: SourceLocation lists/input.md 2:2-2:5
								}
							]
							loc: SourceLocation lists/input.md 2:2-2:5
						}
					]
					loc: SourceLocation lists/input.md 2:2-2:5
				}
			]
			ordered: false
			loc: SourceLocation lists/input.md 2:0-2:5
		}
		MarkdownListBlock {
			children: [
				MarkdownListItem {
					children: [
						MarkdownParagraph {
							children: [
								MarkdownText {
									value: "foo"
									loc: SourceLocation lists/input.md 4:3-4:6
								}
							]
							loc: SourceLocation lists/input.md 4:3-4:6
						}
					]
					loc: SourceLocation lists/input.md 4:3-4:6
				}
			]
			ordered: true
			loc: SourceLocation lists/input.md 4:0-4:6
		}
		MarkdownListBlock {
			children: [
				MarkdownListItem {
					children: [
						MarkdownParagraph {
							children: [
								MarkdownText {
									value: "bar"
									loc: SourceLocation lists/input.md 5:3-5:6
								}
							]
							loc: SourceLocation lists/input.md 5:3-5:6
						}
					]
					loc: SourceLocation lists/input.md 5:3-5:6
				}
			]
			ordered: true
			loc: SourceLocation lists/input.md 5:0-5:6
		}
		MarkdownListBlock {
			children: [
				MarkdownListItem {
					children: [
						MarkdownParagraph {
							children: [
								MarkdownText {
									value: "example"
									loc: SourceLocation lists/input.md 6:3-6:10
								}
							]
							loc: SourceLocation lists/input.md 6:3-6:10
						}
					]
					loc: SourceLocation lists/input.md 6:3-6:10
				}
			]
			ordered: true
			loc: SourceLocation lists/input.md 6:0-6:10
		}
		MarkdownListBlock {
			children: [
				MarkdownListItem {
					value: "-"
					checked: false
					children: [
						MarkdownParagraph {
							children: [
								MarkdownText {
									value: "foo"
									loc: SourceLocation lists/input.md 8:6-8:9
								}
							]
							loc: SourceLocation lists/input.md 8:6-8:9
						}
					]
					loc: SourceLocation lists/input.md 8:6-8:9
				}
			]
			ordered: false
			loc: SourceLocation lists/input.md 8:0-8:9
		}
		MarkdownListBlock {
			children: [
				MarkdownListItem {
					value: "-"
					checked: true
					children: [
						MarkdownParagraph {
							children: [
								MarkdownText {
									value: "bar"
									loc: SourceLocation lists/input.md 9:6-9:9
								}
							]
							loc: SourceLocation lists/input.md 9:6-9:9
						}
					]
					loc: SourceLocation lists/input.md 9:6-9:9
				}
			]
			ordered: false
			loc: SourceLocation lists/input.md 9:0-9:9
		}
		MarkdownListBlock {
			children: [
				MarkdownListItem {
					value: "-"
					checked: true
					children: [
						MarkdownParagraph {
							children: [
								MarkdownText {
									value: "lorem"
									loc: SourceLocation lists/input.md 10:6-10:11
								}
							]
							loc: SourceLocation lists/input.md 10:6-10:11
						}
					]
					loc: SourceLocation lists/input.md 10:6-10:11
				}
			]
			ordered: false
			loc: SourceLocation lists/input.md 10:0-10:11
		}
		MarkdownListBlock {
			children: [
				MarkdownListItem {
					value: "-"
					children: [
						MarkdownParagraph {
							children: [
								MarkdownText {
									value: "["
									loc: SourceLocation lists/input.md 11:3-11:4
								}
								MarkdownText {
									value: "]"
									loc: SourceLocation lists/input.md 11:3-11:4
								}
								MarkdownText {
									value: " something"
									loc: SourceLocation lists/input.md 11:4-11:14
								}
							]
							loc: SourceLocation lists/input.md 11:2-11:14
						}
					]
					loc: SourceLocation lists/input.md 11:2-11:14
				}
			]
			ordered: false
			loc: SourceLocation lists/input.md 11:0-11:14
		}
	]
	comments: []
	corrupt: false
	diagnostics: []
	path: UIDPath<lists/input.md>
	loc: SourceLocation lists/input.md 1:0-11:14
}
```

### `diagnostics`

```

```