from pathlib import Path
from xml.sax.saxutils import escape
import zipfile


ROWS = [
    [
        "Day1_Solution_2025.rs",
        4,
        "The control flow is easy to follow, but magic constants and all-in-main parsing keep it short of excellent.",
        2,
        "Everything from input handling to scoring and timing lives in main with no separation of concerns.",
        3,
        "The code is valid straightforward Rust, but it leans on string slicing, match branches, and panic-based handling rather than richer abstractions.",
        5,
        "The solution is lean and direct, with no noticeable extra machinery.",
    ],
    [
        "Day1b_Solution_2025.rs",
        4,
        "The helper makes the counting idea clear, though the branching and repeated movement logic still add some friction.",
        3,
        "Extracting the zero-counting helper improves the shape, but parsing and state updates still remain packed into main.",
        3,
        "The code uses ordinary Rust patterns appropriately, but it stays fairly manual in both direction handling and state updates.",
        4,
        "The helper keeps the approach focused, though the duplicated direction logic adds a bit of avoidable overhead.",
    ],
    [
        "Day2_Solution_2025.rs",
        4,
        "Names and comments make the brute-force generation easy to follow, even with several nested loops.",
        3,
        "The work is staged sensibly, but the parser and generator are still embedded directly in main.",
        4,
        "Iterator-based parsing, ranges, and numeric helpers are used in a conventional Rust style.",
        4,
        "The approach is straightforward, although repeated range scanning inside the generation loop is slightly heavier than necessary.",
    ],
    [
        "Day2b_Solution_2025.rs",
        4,
        "The comments and naming keep a fairly dense nested enumeration understandable.",
        3,
        "The phases are ordered cleanly, but the whole solution still sits in one function without smaller units.",
        4,
        "HashSet usage and iterator-based parsing are idiomatic choices for this kind of enumeration task.",
        3,
        "Enumerating every repeated-pattern candidate and deduplicating with a set works, but it adds more machinery than the underlying task seems to require.",
    ],
    [
        "Day6_Solution_2025.rs",
        4,
        "The comments and variable names explain the grid grouping logic well, though there is a fair amount of indexing and bookkeeping to scan through.",
        3,
        "The algorithm has clear phases, but they are all implemented inline in main rather than in focused helpers.",
        4,
        "Slices, iterators, match, and collection usage all fit normal Rust practice here.",
        4,
        "The implementation is fairly direct, with only moderate bookkeeping needed to identify groups and parse values.",
    ],
    [
        "Day6b_Solution_2025.rs",
        4,
        "The right-to-left column interpretation is explained clearly, but the grid bookkeeping still makes the body somewhat bulky.",
        3,
        "Like part a, it is logically staged but not decomposed into reusable or testable helpers.",
        4,
        "The solution uses standard Rust collections and iterator chains in a reasonable way.",
        4,
        "The approach is direct for the problem, with some unavoidable indexing but no real overengineering.",
    ],
    [
        "Day7_Solution_2025.rs",
        4,
        "The simulation is short, well named, and easy to trace from top to bottom.",
        3,
        "The workflow is clear, but all search and simulation logic remains inside main.",
        4,
        "Modeling active beams with a HashSet and updating per row is a sensible Rust approach.",
        4,
        "The logic stays focused on the problem without introducing unnecessary abstraction.",
    ],
    [
        "Day7b_Solution_2025.rs",
        4,
        "The counted-beam variant remains readable, and the comments make the HashMap-based state clear.",
        3,
        "The program has a clean top-level flow, but it still lacks any functional decomposition beyond main.",
        4,
        "HashMap::entry accumulation is an idiomatic Rust fit for the counting variant.",
        4,
        "It adds only the state needed to preserve multiplicities, without drifting into overengineering.",
    ],
    [
        "Day11_Solution_2025.rs",
        4,
        "The recursive memoized traversal is compact and clear, though the String-based graph representation is a little noisy.",
        4,
        "The path-counting logic is separated into a focused helper, giving the program a solid top-level structure.",
        4,
        "Recursive DFS with memoization and iterator summation fits Rust reasonably well for this problem.",
        4,
        "The approach is direct, with only minor overhead from owned String keys in the memo and graph.",
    ],
    [
        "Day11b_Solution_2025.rs",
        3,
        "The core idea is recoverable, but the long parameter list, inline id-assignment closure, and bitflag state make it harder to scan quickly.",
        4,
        "Graph construction, stateful path counting, and final orchestration are separated better than in the more monolithic solutions.",
        4,
        "Using integer node ids, bitflags, and memoization is a solid Rust-friendly way to solve the constrained-state variant.",
        3,
        "The solution is effective, but the state plumbing and repeated string allocation in parsing make it more involved than it ideally needs to be.",
    ],
    [
        "Day12_Solution_2025.rs",
        3,
        "The intent is understandable, but the nested peekable parsing and repeated match-and-continue branches create noticeable reading friction.",
        3,
        "The code is split into shape parsing and region evaluation phases, yet both phases remain deeply inline in main.",
        3,
        "The solution is valid Rust, but the manual peekable-loop parser is more verbose and less idiomatic than a cleaner parsing abstraction would be.",
        3,
        "The parsing logic works, but it is more tangled and control-flow heavy than the problem likely requires.",
    ],
]


def col_name(index: int) -> str:
    result = ""
    while index > 0:
        index, rem = divmod(index - 1, 26)
        result = chr(65 + rem) + result
    return result


def cell_xml(col: int, row: int, value) -> str:
    ref = f"{col_name(col)}{row}"
    if isinstance(value, int):
        return f'<c r="{ref}"><v>{value}</v></c>'
    return f'<c r="{ref}" t="inlineStr"><is><t>{escape(str(value))}</t></is></c>'


def build_sheet_xml(rows: list[list[object]]) -> str:
    xml_rows = []
    for row_idx, row in enumerate(rows, start=1):
        cells = "".join(cell_xml(col_idx, row_idx, value) for col_idx, value in enumerate(row, start=1))
        xml_rows.append(f'<row r="{row_idx}">{cells}</row>')
    return (
        '<?xml version="1.0" encoding="UTF-8" standalone="yes"?>'
        '<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">'
        f"<sheetData>{''.join(xml_rows)}</sheetData>"
        "</worksheet>"
    )


def main() -> None:
    header = [
        "File",
        "Readability",
        "Readability Justification",
        "Structure",
        "Structure Justification",
        "Idiomatic Use",
        "Idiomatic Justification",
        "Unnecessary Complexity",
        "Complexity Justification",
    ]
    rows = [header, *ROWS]

    output = Path("LLM-as-a-judge_2025.xlsx")
    with zipfile.ZipFile(output, "w", compression=zipfile.ZIP_DEFLATED) as zf:
        zf.writestr(
            "[Content_Types].xml",
            """<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/>
  <Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/>
</Types>
""",
        )
        zf.writestr(
            "_rels/.rels",
            """<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="xl/workbook.xml"/>
</Relationships>
""",
        )
        zf.writestr(
            "xl/workbook.xml",
            """<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <sheets>
    <sheet name="Scores" sheetId="1" r:id="rId1"/>
  </sheets>
</workbook>
""",
        )
        zf.writestr(
            "xl/_rels/workbook.xml.rels",
            """<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/>
</Relationships>
""",
        )
        zf.writestr("xl/worksheets/sheet1.xml", build_sheet_xml(rows))


if __name__ == "__main__":
    main()
