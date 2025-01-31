Crate kuzuCopy item path
Settings
Help

Summary
Source
Bindings to Kùzu: an in-process property graph database management system built for query speed and scalability.

Example Usage
use kuzu::{Database, SystemConfig, Connection};

let db = Database::new(path, SystemConfig::default())?;
let conn = Connection::new(&db)?;
conn.query("CREATE NODE TABLE Person(name STRING, age INT64, PRIMARY KEY(name));")?;
conn.query("CREATE (:Person {name: 'Alice', age: 25});")?;
conn.query("CREATE (:Person {name: 'Bob', age: 30});")?;

let mut result = conn.query("MATCH (a:Person) RETURN a.name AS NAME, a.age AS AGE;")?;
println!("{}", result.display());
Building
By default, the kuzu C++ library will be compiled from source and statically linked.

If you want to instead link against a pre-built version of the library, the following environment variables can be used to configure the build process:

KUZU_SHARED: If set, link dynamically instead of statically
KUZU_INCLUDE_DIR: Directory of kuzu’s headers
KUZU_LIBRARY_DIR: Directory containing kuzu’s pre-built libraries.
Structs
ArrowIterator	Produces an iterator over a QueryResult as RecordBatches
CSVOptions	Options for writing CSV files
Connection	Connections are used to interact with a Database instance.
Database	The Database class is the main class of KuzuDB. It manages all database components.
InternalID	Stores the table_id and offset of a node/rel.
NodeVal	NodeVal represents a node in the graph and stores the nodeID, label and properties of that node.
PreparedStatement	A prepared stattement is a parameterized query which can avoid planning the same query for repeated execution
QueryResult	Stores the result of a query execution
RelVal	RelVal represents a relationship in the graph and stores the relID, src/dst nodes and properties of that rel
SystemConfig	Configuration options for the database.
Enums
Error
LogicalType	Type of Values produced and consumed by queries.
Value	Data types supported by Kùzu
Constants
VERSION	The version of the Kùzu crate as reported by Cargo’s CARGO_PKG_VERSION environment variable
Functions
get_storage_version	Returns the storage version of the Kùzu library



KUZU CYPHER MANUAL:

Cypher manual
Cypher is a high-level query language for the property graph data model. If you’re coming from a SQL background, it’s syntax may seem familiar. Some common analogues between SQL and Cypher are listed below:

Type	SQL	Cypher
Query	SELECT/FROM/WHERE	MATCH/WHERE/RETURN
Data manipulation	INSERT/UPDATE/DELETE	CREATE/SET/DELETE
The features of Cypher that are different from SQL are listed below:

Joins between records from different node and relationship tables are specified using a graph-like syntax.
MATCH (n:Person)-[:Follows]->(m:Person)
Special syntax, such as the Kleene star * to describe variable-length and recursive joins.
Cypher does not have an explicit GROUP BY like SQL does — instead, grouping is done implicitly depending on the combination of bound variables in the RETURN clause.
There are a few other differences between SQL and Cypher. Yet, like other high-level database query languages, most of its semantics can be understood as mappings to relational algebra operators for selections, joins, projections and aggregations.

Kùzu implements openCypher’s1 standard predicates and expressions. The following sections in this chapter covers all Cypher statements, clauses, expressions and functions implemented in Kùzu.

Statements vs. clauses
In Cypher, a statement is a complete query that can be executed on its own. A statement can contain one or more clauses, and can span multiple lines. The end of a statement is marked with a semicolon ;, and the query parser looks for this symbol to know when a statement is complete.

A clause is a part of a statement that performs a specific operation. For example, the MATCH clause is used to find patterns in the graph, the RETURN clause is used to specify what subset of the matched data to return, and so on.

Multi-line statements
You can execute multiple query statements sequentially in the CLI, rather than executing them one by one. To run a multi-line statements, simply end each valid statement you want to execute with a semicolon ;. For example:

MATCH (p1:Person)
WHERE p1.age <= 18
RETURN p1.name AS non_adult;

MATCH (p2:Person)
WHERE p2.age > 18
RETURN p2.name AS adult;

When you copy-paste the above blocks into the Kùzu CLI and press Enter, it will execute each block sequentially, so you don’t have to send individual queries one at a time.


Syntax
In this page, we list the syntactic features of Cypher as implemented in Kùzu. As described in the overview page, Cypher is a declarative graph query language, and Kùzu’s implementation is based on openCypher.

Parsing
Encoding
The Cypher query parser looks for an input STRING that consists of ASCII or unicode characters from non-English languages. An example is shown below for creating and querying from a node table of German books.

// Create a node table of books in German
CREATE NODE TABLE Bücher (title STRING, price INT64, PRIMARY KEY (title))
CREATE (n:Bücher {title: 'Der Thron der Sieben Königreiche'}) SET n.price = 20
// Query using the unicode representation of the table name
MATCH (n:Bücher) RETURN label(n)

┌─────────┐
│ Bücher  │
│ STRING  │
├─────────┤
│ Bücher  │
└─────────┘

Escaping
To use special characters in identifiers, you can escape them by encapsulating the identifier in backticks `. An example is shown below for creating a node table of house names that contain special characters.

// Create a node table of house names that contain special characters
CREATE NODE TABLE `HouseΨ` (id INT64, member STRING, PRIMARY KEY (id))
CREATE (n:`HouseΨ` {id: 1}) SET n.member = 'Alice'
// Query on the unicode table name
MATCH (n:`HouseΨ`) RETURN n.*

┌───────┬──────────┐
│ n.id  │ n.member │
│ INT64 │ STRING   │
├───────┼──────────┤
│ 1     │ Alice    │
└───────┴──────────┘

Multiline statements and termination
Breaking a query into multiple lines is allowed (and recommended for readability reasons). The query parser ignores leading and trailing whitespaces.

MATCH (a:Person)
WHERE a.age < 30
RETURN a.*;

Termination is always indicated by a semicolon ;, and the parser looks for this symbol to know when a statement is complete.

Clauses
A Cypher query may contain one or more clauses and their associated subclauses, and can span multiple lines. The end of a statement is marked with a semicolon ;, and the query parser looks for this symbol to know when a statement is complete.

Examples of clauses include:

MATCH: Find patterns in the graph
RETURN: Specify what subset of the matched data to return
Examples of subclauses (that must reside under a clause) include:

WHERE: Filter the results of a MATCH clause
LIMIT: Limit the number of results returned by a query
Comments
Comments are for humans to read and document their code, and are ignored by the query parser.

Single line comments begin with a double slash (//) and continue up until the end of the line. They can be placed at the beginning, in the middle, or at the end of a query.
Multi-line comments begins with a slash and asterisk (/*) and continues until it ends with an asterisk and a slash (*/). They can be useful for comments that are too long for one line.
Some examples are below.

// Whole-line comment before a query
MATCH (a:Person) RETURN a.*

MATCH (a:Person) RETURN a.*  // Comment at the end of a query

MATCH (a:Person)
// Comment in the middle of a query
WHERE a.age < 30
RETURN a.*

/*
This is a comment
spanning multiple lines
*/
MATCH (a:Person) RETURN a.*

Naming rules and recommendations
As a general rule of thumb, ensure the following:

Names should begin with an valid alphabetic character of type unicode string — Person, CarOwner
Names should not begin with a number — 1Person is invalid, but Person1 is valid
Names should not contain whitespaces or special characters other than underscores — CarOwner is valid, but Car Owner is invalid
Names are generally case-insensitive — Person is the same as person, during table creation and querying
The following naming conventions are recommended for node and relationship tables:

Type	Naming convention	Do	Don’t
Node tables	CamelCase (begin with upper case letter)	CarOwner	car_owner
Relationship tables	CamelCase or UPPERCASE separated by underscores	IsPartOf/IS_PART_OF	isPartOf or is_part_of
Parameters
Parameters in Cypher queries are placeholders for values that are provided at runtime. Parameters are prefixed with a dollar sign $ and can be used in any part of a query. They are useful for preventing Cypher injection attacks, and for reusing query templates with different values.

See the prepared statements guide for more information on how to use parameters in Kùzu.

Reserved keywords
Reserved keywords are words that have a special meaning in Cypher. They cannot be used as identifiers in the following contexts:

Variables
Function names
Parameters
To use a reserved keyword as an identifier in the above contexts, you can escape it by encapsulating the keyword in backticks `, such as `DEFAULT`, and this makes it a valid identifier.

The following list shows the reserved keywords in Cypher, organized by category:

Clauses
COLUMN
CREATE
DBTYPE
DEFAULT
GROUP
HEADERS
INSTALL
MACRO
OPTIONAL
PROFILE
UNION
UNWIND
WITH
Subclauses
LIMIT
ONLY
ORDER
WHERE
Expressions
ALL
CASE
CAST
ELSE
END
ENDS
EXISTS
GLOB
SHORTEST
THEN
WHEN
Literals
NULL
FALSE
TRUE
Modifiers
ASC
ASCENDING
DESC
DESCENDING
ON
Operators
AND
DISTINCT
IN
IS
NOT
OR
STARTS
XOR
Schema
FROM
PRIMARY
TABLE
TO


Data types
Kùzu supports a set of primitive and nested data types both for node and relationship properties as well as for forming expressions whose outputs are specified using these data types. This section shows all built-in data types.

INT8
Size	Description
1 byte	signed one-byte integer
INT16
Size	Description
2 bytes	signed two-byte integer
INT32
Size	Description	Aliases
4 bytes	signed four-byte integer	INT
INT64
Size	Description	Aliases
8 bytes	signed eight-byte integer	SERIAL
INT128
Size	Description
16 bytes	signed sixteen-byte integer
UINT8
Size	Description
1 byte	unsigned one-byte integer
UINT16
Size	Description
2 bytes	unsigned two-byte integer
UINT32
Size	Description
4 bytes	unsigned four-byte integer
UINT64
Size	Description
8 bytes	unsigned eight-byte integer
FLOAT
Size	Description	Aliases
4 bytes	single precision floating-point number	REAL, FLOAT4
DOUBLE
Size	Description	Aliases
8 bytes	double precision floating-point number	FLOAT8
DECIMAL
Size	Description
variable	arbitrary fixed precision decimal number
For numbers where exact precision is required, the DECIMAL data type can be used. The DECIMAL type is specified as DECIMAL(precision, scale), where precision is the total number of digits and scale is the number of digits to the right of the decimal point.

Internally, decimals are represented as integers depending on their specified width.

Precision	Internal	Size (bytes)
1-4	INT16	2
5-9	INT32	4
10-18	INT64	8
19-38	INT128	16
You can explicitly cast a number (either integer or float) to a DECIMAL as follows:

RETURN CAST(127.3, "DECIMAL(5, 2)") AS result;

Output:

┌───────────────┐
│ result        │
│ DECIMAL(5, 2) │
├───────────────┤
│ 127.30        │
└───────────────┘

Note that if you attempt to cast with a precision or scale that is too small, an overflow exception will be raised:

RETURN CAST(127.3, "DECIMAL(4, 2)");

Error: Overflow exception: To Decimal Cast Failed: 127.300000 is not in DECIMAL(4, 2) range

BOOLEAN
Size	Description
1 byte	true/false
UUID
Size	Description
16 bytes	signed sixteen-byte integer
The data type UUID stores Universally Unique Identifiers (UUID) as defined by RFC 4122, ISO/IEC 9834-8:2005, and related standards. Kuzu follows PostgreSQL’s implementation for the UUID format.

Example:

RETURN UUID('A0EEBC99-9C0B-4EF8-BB6D-6BB9BD380A11') as result;

Output:

┌──────────────────────────────────────┐
│ result                               │
│ UUID                                 │
├──────────────────────────────────────┤
│ a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11 │
└──────────────────────────────────────┘

STRING
Size	Description
variable	variable-length character string
STRING data type supports UTF-8 encoding.

Example:

RETURN 'Зарегистрируйтесь, σπαθιοῦ, Yen [jɛn], kΩ' AS str;

Output:

┌───────────────────────────────────────────┐
│ str                                       │
│ STRING                                    │
├───────────────────────────────────────────┤
│ Зарегистрируйтесь, σπ...                  │
└───────────────────────────────────────────┘

NULL
Size	Description
fixed	special value to represent unknown data
NULLs are special values to represent unknown data. Every node/relationship property or result of any expression can be NULL in addition to the non-NULL domain of values they can take. For example, boolean expressions can be true, false or NULL.

The NULL (in any of its case variations, such as Null or null) can be used to specify a null literal. Some examples of comparisons using NULL are shown below.

Compare a value with NULL:

RETURN 3 = null;

Output:

┌────────────┐
│ EQUALS(3,) │
│ BOOL       │
├────────────┤
│            │
└────────────┘

Compare NULL with NULL:

RETURN null = null;

Output:

┌───────────┐
│ EQUALS(,) │
│ BOOL      │
├───────────┤
│           │
└───────────┘

Kùzu’s CLI returns an empty cell to indicate nulls.

DATE
Size	Description
4 bytes	year, month, day
DATE is specified in ISO-8601 format (YYYY-MM-DD).

Example:

RETURN date('2022-06-06') as x;

Output:

┌────────────┐
│ x          │
│ DATE       │
├────────────┤
│ 2022-06-06 │
└────────────┘

TIMESTAMP
Size	Description
4 bytes	combination of time and date
TIMESTAMP combines date and a time (hour, minute, second, millisecond) and is formatted according to the ISO-8601 format (YYYY-MM-DD hh:mm:ss[.zzzzzz][+-TT[:tt]]), which specifies the date (YYYY-MM-DD), time (hh:mm:ss[.zzzzzz]) and a time offset [+-TT[:tt]]. Only the Date part is mandatory. If time is specified, then the millisecond [.zzzzzz] part and the time offset are optional.

Example:

RETURN timestamp("1970-01-01 00:00:00.004666-10") as x;

Output:

┌────────────────────────────┐
│ x                          │
│ TIMESTAMP                  │
├────────────────────────────┤
│ 1970-01-01 10:00:00.004666 │
└────────────────────────────┘

INTERVAL
Size	Description	Aliases
4 bytes	date/time difference	DURATION
INTERVAL consists of multiple date parts and represents the total time length of these date parts. Kùzu follows DuckDB’s implementation for the interval format.

Example:

RETURN interval("1 year 2 days") as x;

Output:

┌───────────────┐
│ x             │
│ INTERVAL      │
├───────────────┤
│ 1 year 2 days │
└───────────────┘

STRUCT
A STRUCT is a mapping of key-value pairs where the keys are of the type STRING. STRUCT is a fixed-size data type so values with the same STRUCT type must contain the same set of key-value pairs. You can think of a STRUCT column as a nested single column over multiple other columns.

Data Type	DDL definition
STRUCT	STRUCT(a INT64, b INT64)
To construct a STRUCT, provide a mapping of keys to values as follows:

RETURN {first: 'Adam', last: 'Smith'};

Output:

┌───────────────────────────────────┐
│ STRUCT_PACK(first,last)           │
│ STRUCT(first STRING, last STRING) │
├───────────────────────────────────┤
│ {first: Adam, last: Smith}        │
└───────────────────────────────────┘

You can extract a value from a STRUCT using the dot notation:

WITH {first: 'Adam', last: 'Smith'} AS full_name
RETURN full_name.first AS first_name;

Output:

┌────────────┐
│ first_name │
│ STRING     │
├────────────┤
│ Adam       │
└────────────┘

Alternatively you can use the struct_extract() function

WITH {first:'Adam', last: 'Smith'} AS full_name
RETURN struct_extract(full_name, 'first') AS first_name;

Functions that work on STRUCTs can be found here.

MAP
A MAP is a dictionary of key-value pairs where all keys have the same type and all values have the same type. MAP is similar to STRUCT in that it is an ordered list of mappings. However, MAP does not need to have the same keys present for each row, and is thus more suitable when the schema of an entity is unknown beforehand or when the schema varies per row.

MAPs must have a single type for all keys, and a single type for all values. Additionally, keys of a MAP do not need to be STRINGs like they do in a STRUCT.

Data Type	DDL definition
MAP	MAP(STRING, INT64)
To construct a MAP, provide a list of keys and a list of values. The keys and values must be of the same length.

Example:

RETURN map([1, 2], ['a', 'b']) AS m;

Output:

┌────────────────────┐
│ m                  │
│ MAP(INT64, STRING) │
├────────────────────┤
│ {1=a, 2=b}         │
└────────────────────┘

Functions that work on map objects can be found here.

UNION
Similar to C++ std::variant, UNION is a nested data type that is capable of holding multiple alternative values with different types. The value under key "tag" is considered as the value being currently hold by the UNION.

Internally, UNION are implemented as STRUCT with "tag" as one of its keys.

Data Type	DDL definition
UNION	UNION(price FLOAT, note STRING)
Consider the following CSV file:

demo.csv
1
aa

Example

CREATE NODE TABLE demo(a SERIAL, b UNION(num INT64, str STRING), PRIMARY KEY(a));
COPY demo from "demo.csv";

MATCH (d:demo) RETURN d.b;

┌──────────────────────────────┐
│ d.b                          │
│ UNION(num INT64, str STRING) │
├──────────────────────────────┤
│ 1                            │
│ aa                           │
└──────────────────────────────┘

Functions that work on UNION data types can be found here.

BLOB
Size	Description	Aliases
variable	arbitrary binary object	BYTEA
BLOB(Binary Large OBject) allows storage of an arbitrary binary object with up to 4KB in size in Kùzu. The database processes it as binary data because it has no knowledge as to what the underlying data represents (e.g. image, video).

Below is an example of how to create a blob object with 3 bytes (188, 189, 186, 170):

RETURN BLOB('\\xBC\\xBD\\xBA\\xAA') as result;

Output:

┌──────────────────┐
│ result           │
│ BLOB             │
├──────────────────┤
│ \xBC\xBD\xBA\xAA │
└──────────────────┘

SERIAL
SERIAL is a logical data type used for creating an auto-incrementing sequence of numbers, typically used as a unique column identifier, similar to AUTO_INCREMENT feature supported by some other databases.

Using SERIAL as primary key column in node tables
person.csv
Alice
Bob
Carol
Dan

CREATE NODE TABLE Person(id SERIAL, name STRING, PRIMARY KEY(id));
COPY Person FROM 'person.csv';
MATCH (a:Person) RETURN a.*;

Output:

┌────────┬────────┐
│ a.id   │ a.name │
│ SERIAL │ STRING │
├────────┼────────┤
│ 0      │ Alice  │
│ 1      │ Bob    │
│ 2      │ Carol  │
│ 3      │ Dan    │
└────────┴────────┘

Using SERIAL for properties in relationship tables
You can create relationship tables that have a SERIAL property column. For example, consider a scenario where you want to auto-generate a unique transaction ID for each transfer between users.

CREATE REL TABLE Transfer (from User to User, trx_id SERIAL);

NODE
Size	Description
fixed	represents a node in a graph
NODE is a logical data type. Internally, NODE is processed as STRUCT type. A NODE always contains an internal ID field with key _ID and a label field with key _LABEL. The rest fields are node properties.

Here’s how to return NODE column for a file person.csv:


CREATE NODE TABLE Person(id SERIAL, name STRING, age INT64, PRIMARY KEY(id));
COPY Person FROM 'person.csv';
MATCH (a:Person) RETURN a;

Output:

┌─────────────────────────────────────────────────────────┐
│ a                                                       │
│ NODE                                                    │
├─────────────────────────────────────────────────────────┤
│ {_ID: 0:0, _LABEL: Person, id: 0, name: Alice, age: 30} │
│ {_ID: 0:1, _LABEL: Person, id: 1, name: Bob, age: 20}   │
│ {_ID: 0:2, _LABEL: Person, id: 2, name: Carol, age: 25} │
│ {_ID: 0:3, _LABEL: Person, id: 3, name: Dan, age: 28}   │
└─────────────────────────────────────────────────────────┘

REL
Size	Description
fixed	represents a relationship in a graph
REL is a logical type that represents a relationship (i.e., an edge). Internally, REL is processed as STRUCT type. A REL always contains a src ID field with key _SRC, a dst ID field with key _DST, an internal ID field with key _ID and a label field with key _LABEL. The rest fields are rel properties.

Here’s how to return a relationship column that’s of type REL:

MATCH (a:Person)-[r:Follows]->(b:Person)
RETURN r;

Output:

┌───────────────────────────────────────────────┐
│ r                                             │
│ REL                                           │
├───────────────────────────────────────────────┤
│ (0:0)-{_LABEL: Follows, _ID: 1:0, since: 2... │
│ (0:1)-{_LABEL: Follows, _ID: 1:1, since: 2... │
│ (0:2)-{_LABEL: Follows, _ID: 1:2, since: 2... │
│ (0:3)-{_LABEL: Follows, _ID: 1:3, since: 2... │
└───────────────────────────────────────────────┘

RECURSIVE_REL
RECURSIVE_REL is a logical type that represents recursive relationships. i.e., paths of arbitrary lengths. Internally, RECURSIVE_REL is processed as STRUCT type, more specifically, a STRUCT{LIST[NODE], LIST[REL]}. A RECURSIVE_REL always contains a nodes field with the key _NODES and a relationships field with the key _RELS.

Return a column that’s of type RECURSIVE_REL
MATCH p = (a:User)-[:Follows]->(b:User)
WHERE a.name = 'Adam' AND b.name = 'Karissa'
RETURN p;

Output:

{_NODES: [{_ID: 0:0, _LABEL: User, name: Adam, age: 30},{_ID: 0:1, _LABEL: User, name: Karissa, age: 40}], _RELS: [(0:0)-{_LABEL: Follows, _ID: 2:0, since: 2020}->(0:1)]}

Access all nodes on a recursive relationship
MATCH p = (a:Person)-[:Follows]->(b:Person)
WHERE a.name = 'Alice' AND b.name = 'Bob'
RETURN nodes(p);

Output:

┌─────────────────────────────────────────────────────────────────────────────────┐
│ NODES(p)                                                                        │
│ NODE[]                                                                          │
├─────────────────────────────────────────────────────────────────────────────────┤
│ [{_ID: 0:0, _LABEL: Person, name: Alice},{_ID: 0:1, _LABEL: Person, name: Bob}] │
└─────────────────────────────────────────────────────────────────────────────────┘

Access all relationships on a recursive relationship
MATCH p = (a:Person)-[:Follows]->(b:Person)
WHERE a.name = 'Alice' AND b.name = 'Bob'
RETURN rels(p);

Output:

┌─────────────────────────────────────────────────────────┐
│ RELS(p)                                                 │
│ REL[]                                                   │
├─────────────────────────────────────────────────────────┤
│ [(0:0)-{_LABEL: Follows, _ID: 1:0, since: 2024}->(0:1)] │
└─────────────────────────────────────────────────────────┘

LIST and ARRAY
Kùzu supports two list-like data types: (i) variable-length lists, simply called LIST, and (ii) fixed-length lists, called ARRAY. Click on the card below to learn more about them.

How to work with LIST and ARRAY
Example usage of LIST and ARRAY data types
JSON
Requires the JSON extension

The JSON data type is not natively available in Kùzu, and requires the JSON extension

Using the Kùzu JSON extension, you can model store properties as JSON natively via the JSON logical type, which is interpreted as parsed as JSON, rather than as a string.


JSON EXTENSION:


JSON extension
Usage
The json extension adds support for JSON objects, including a set of functions for JSON access and manipulation, scanning from, and copying to JSON files. Using this extension, you can interact with JSON files using LOAD FROM, COPY FROM, and COPY TO, similar to how you would with CSV files.

The JSON functionality is not available by default, so you would first need to install the JSON extension by running the following commands:

INSTALL json;
LOAD EXTENSION json;

See our YouTube video for a walkthrough on how to use the JSON extension:


Example dataset
Let’s look at an example dataset to demonstrate how the JSON extension can be used. We have 3 JSON files that contain information about patients and their medical conditions. The files are organized into two node files (patient.json and condition.json) and one relationship file (has_condition.json).

patient.json
condition.json
has_condition.json
[
    {
        "p_id": "p1",
        "name": "Gregory",
        "info": {
            "height": 1.81,
            "weight": 75.5,
            "age": 35,
            "insurance_provider": [
                {
                    "type": "health",
                    "name": "Blue Cross Blue Shield",
                    "policy_number": "1536425345"
                },
                {
                    "type": "dental",
                    "name": "Cigna dental",
                    "policy_number": "745332412"
                }
            ]
        }
    },
    {
        "p_id": "p2",
        "name": "Alicia",
        "info": {
            "height": 1.65,
            "weight": 60.1,
            "age": 28,
            "insurance_provider": [
                {
                    "type": "health",
                    "name": "Aetna",
                    "policy_number": "9876543210"
                }
            ]
        }
    },
    {
        "p_id": "p3",
        "name": "Rebecca"
    }
]

In the following sections, we will first scan the JSON files to query its contents in Cypher, and then proceed to copy the JSON data and construct a graph.

Scan the JSON file
LOAD FROM is a Cypher query that scans a file or object element by element, but doesn’t actually move the data into a Kùzu table.

Because the JSON format contains simple data types without type information, the structure will be inferred. To declare type information explicitly, you can use LOAD WITH HEADERS like you would for CSV files.

To scan the file above, you can do the following:

LOAD FROM 'patient.json' RETURN *;

┌────────┬─────────┬──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐
│ p_id   │ name    │ info                                                                                                                                                                                                     │
│ STRING │ STRING  │ STRUCT(height DOUBLE, weight DOUBLE, age UINT8, insurance_provider STRUCT(type STRING, name STRING, policy_number STRING)[])                                                                             │
├────────┼─────────┼──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ p1     │ Gregory │ {height: 1.810000, weight: 75.500000, age: 35, insurance_provider: [{type: health, name: Blue Cross Blue Shield, policy_number: 1536425345},{type: dental, name: Cigna dental, policy_number: 7453324... │
│ p2     │ Alicia  │ {height: 1.650000, weight: 60.100000, age: 28, insurance_provider: [{type: health, name: Aetna, policy_number: 9876543210},{type: vision, name: VSP, policy_number: 1784567890}]}                        │
│ p3     │ Rebecca │ {height: 1.780000, weight: , age: 23, insurance_provider: [{type: health, name: Blue Cross Blue Shield, policy_number: 5678901234}]}                                                                     │
└────────┴─────────┴──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘

Because info is a nested object, its type in Kùzu is inferred as a STRUCT, that itself contains other types, like DOUBLE, UINT8, STRING, and STRUCT.

Missing keys
Missing keys, i.e., keys that are present in one JSON blob but not in another, are returned as the default/empty value for the type. To test this, let’s run another query to get the name, age, height, weight and insurance provider of all patients:

LOAD FROM 'patient.json' RETURN name, info.age, info.height, info.weight, info.insurance_provider;

┌─────────┬──────────────────────────┬─────────────────────────────┬─────────────────────────────┬─────────────────────────────────────────────────────────────────┐
│ name    │ STRUCT_EXTRACT(info,age) │ STRUCT_EXTRACT(info,height) │ STRUCT_EXTRACT(info,weight) │ STRUCT_EXTRACT(info,insurance_provider)                         │
│ STRING  │ UINT8                    │ DOUBLE                      │ DOUBLE                      │ STRUCT(type STRING, name STRING, policy_number STRING)[]        │
├─────────┼──────────────────────────┼─────────────────────────────┼─────────────────────────────┼─────────────────────────────────────────────────────────────────┤
│ Gregory │ 35                       │ 1.810000                    │ 75.500000                   │ [{type: health, name: Blue Cross Blue Shield, policy_number:... │
│ Alicia  │ 28                       │ 1.650000                    │ 60.100000                   │ [{type: health, name: Aetna, policy_number: 9876543210}]        │
│ Rebecca │ 0                        │ 0.000000                    │ 0.000000                    │ []                                                              │
└─────────┴──────────────────────────┴─────────────────────────────┴─────────────────────────────┴─────────────────────────────────────────────────────────────────┘

As can be seen, the patient Rebecca is new in the system and is missing her information fields:

age is set to the default value of 0 for UINT8
height and weight are set to the default value of 0.0 for DOUBLE
insurance_provider is set to an empty array []
Enforcing types
To enforce the data type during scanning, use the LOAD WITH HEADERS feature.

Example:

LOAD WITH HEADERS (
    p_id STRING,
    name STRING,
    info STRUCT(
        height FLOAT,
        weight FLOAT,
        age UINT8,
        insurance_provider STRUCT(type STRING, name STRING, policy_number STRING)[]
    )
)
FROM 'patient.json'
RETURN name, info.height, info.weight;

We can see that the types inside the info STRUCT are now enforced to FLOAT, rather than DOUBLE.

┌─────────┬─────────────────────────────┬─────────────────────────────┐
│ name    │ STRUCT_EXTRACT(info,height) │ STRUCT_EXTRACT(info,weight) │
│ STRING  │ FLOAT                       │ FLOAT                       │
├─────────┼─────────────────────────────┼─────────────────────────────┤
│ Gregory │ 1.810000                    │ 75.500000                   │
│ Alicia  │ 1.650000                    │ 60.099998                   │
│ Rebecca │ 0.000000                    │ 0.000000                    │
└─────────┴─────────────────────────────┴─────────────────────────────┘

Optional parameters
The following optional parameters are supported:

Name	Description
maximum_depth	Default value is 10. Used by the type inference system to determine how “deep” into the json document to go to infer types.
sample_size	Default value 2048. Used by the type inference system to determine the number of elements used to infer the json type.
Copy JSON files to a table
The COPY FROM statement allows you to copy data from a JSON file into a node or relationship table in Kùzu. In this section we will walk through the example dataset shown above and build a graph from the JSON data.

Copy to node tables
First, start by defining a node table schema that conforms to the JSON structure. For nested fields, we declare a STRUCT where necessary.

Example:

CREATE NODE TABLE IF NOT EXISTS Patient(
    p_id STRING,
    name STRING,
    info STRUCT(
        height FLOAT,
        weight FLOAT,
        age UINT8,
        insurance_provider STRUCT(
            type STRING,
            name STRING,
            policy_number STRING
        )[]
    ),
    PRIMARY KEY (p_id)
)

The syntax STRUCT( ... )[] with the square braces at the end represents an arrya of STRUCTs.

You can then use a COPY FROM statement to directly copy the contents of the JSON file into the node table.

COPY Patient FROM 'patient.json'

Similarly, we can define the node table for the patients’ medical conditions.

CREATE NODE TABLE IF NOT EXISTS Condition(
    c_id STRING,
    name STRING,
    description STRING,
    PRIMARY KEY (c_id)
)

And copy the contents of condition.json to the node table as follows:

conn.execute("COPY Condition FROM 'condition.json'")

Copy to relationship tables
To copy from a JSON file to a relationship table, the file must contain the "from" and "to" keys.

In the example dataset for has_condition.json, we have these keys defined:

[
    {
        "from": "p1",
        "to": "c1",
        "since": 2019
    },
    {
        "from": "p1",
        "to": "c2",
        "since": 2015
    },
    ...
]

Any other keys that are not "from" or "to" are treated as relationship properties.

Let’s create a relationship table schema:

CREATE REL TABLE IF NOT EXISTS HAS_CONDITION(
    FROM Patient TO Condition,
    since UINT16
)

The has_condition.json file can then directly be copied into the relationship table that was just created.

COPY HAS_CONDITION FROM 'has_condition.json'

We obtain the following graph:


Any nested fields are ingested into the graph as STRUCTs. We can query on these nested fields as shown below:

MATCH (p:Patient)-[:HAS_CONDITION]->(c:Condition)
WHERE c.name = "Diabetes (Type 1)"
WITH p.name AS name, p.info.age AS age, c.name AS condition, p.info.insurance_provider AS ip
UNWIND ip AS provider
WITH name, age, provider, condition
WHERE provider.type = "health"
RETURN name, age, condition, provider.name AS health_insurance_provider

┌─────────┬───────┬───────────────────┬───────────────────────────┐
│ name    │ age   │ condition         │ health_insurance_provider │
│ STRING  │ UINT8 │ STRING            │ STRING                    │
├─────────┼───────┼───────────────────┼───────────────────────────┤
│ Gregory │ 35    │ Diabetes (Type 1) │ Blue Cross Blue Shield    │
│ Alicia  │ 28    │ Diabetes (Type 1) │ Aetna                     │
└─────────┴───────┴───────────────────┴───────────────────────────┘

Note how the UNWIND clause was used to obtain individual records of the insurance providers for each patient.

UNWIND JSON arrays
In the above example, we have useful information about insurance providers that could also be used to capture the relationships between patients and their insurance providers.

Let’s model this using a new node table, InsuranceProvider, and a new relationship table HAS_PROVIDER.

CREATE NODE TABLE IF NOT EXISTS InsuranceProvider(
    name STRING,
    type STRING,
    PRIMARY KEY (name)
)

CREATE REL TABLE IF NOT EXISTS HAS_PROVIDER(
    FROM Patient TO InsuranceProvider,
    policy_number STRING
)

We can then UNWIND the insurance providers for each patient, obtain distinct providers, and then pass these results via a subquery to COPY FROM.

COPY InsuranceProvider FROM (
    LOAD FROM 'patient.json'
    WITH info.insurance_provider AS ip
    UNWIND ip AS provider
    RETURN DISTINCT
        provider.name AS name,
        provider.type AS type
)

Let’s break down the above query step by step:

The outer COPY FROM expects the result from the inner LOAD FROM
The info STRUCT from patient.json is passed to UNWIND so that we can obtain individual providers for each patient
A DISTINCT clause is used when returning the results of the subquery, because the name of a provider is the primary key of the InsuranceProvider node table per the schema created above (we cannot have duplicate values for primary keys).
We can do a similar sequence of steps to copy relationships from patient.json as follows:

COPY HAS_PROVIDER FROM (
    LOAD FROM 'patient.json'
    WITH p_id, info.insurance_provider AS ip
    UNWIND ip AS provider
    RETURN
        p_id,
        provider.name AS name,
        provider.policy_number AS policy_number
)

In this case, we didn’t alias the first two entries to from and to, like we did when copying from the has_condition.json file above. This is because the COPY FROM query is looking for the first two columns in the result as the FROM and the TO columns in the relationship, similar to how it’s done in CSV.

We now obtain the following graph:


Copy query results to JSON files
Once you have the data in a graph, you can begin querying it in Cypher. You can use the COPY TO statement to write the results of a query to a JSON file. Any query results of the type STRUCT will be written as nested JSON. Two examples are shown below.

Example 1
Example 2
Say you want to write health insurance provider information and patient names for patients with the condition “Migraine” to a JSON file named patient_providers.json.

COPY (
    MATCH (p:Patient)-[:HAS_CONDITION]->(c:Condition)
    WHERE c.name = "Migraine"
    WITH p.name AS name, p.info.age AS age, c.name AS condition, p.info.insurance_provider AS ip
    UNWIND ip AS provider
    WITH name, age, provider, condition
    WHERE provider.type = "health"
    RETURN name, age, condition, provider
) TO 'patient_providers.json';

The output JSON would look like this:

[
    {
        "name": "Alicia",
        "age": 28,
        "condition": "Migraine",
        "provider": {
            "type": "health",
            "name": "Aetna",
            "policy_number": "9876543210"
        }
    }
]

Takeaways
When using the JSON extension, keep in mind the following considerations when copying data to Kùzu tables:

The order of the keys in the JSON file doesn’t need to match with the order of the columns defined in the schema (just the names need to match)

If directly copying from a JSON file to a relationship table, there need to be keys named "from" and "to" in the file, whose values point to the primary key values of the underlying node tables.

You can combine LOAD FROM subqueries with COPY FROM to have more control over the subset of JSON data being copied, as well as dynamically transform your data via UNWIND or DISTINCT clauses, so it’s not necessary to write your relationships to an intermediate file prior to using COPY.

JSON data type
Using the Kùzu JSON extension, you can model and store properties as JSON natively via the JSON logical type, which is interpreted as parsed as JSON, rather than as a string.

The following example creates a node table Person with a JSON column description, it then creates two json objects in this column using to_json function, and outputs them.

Example:

INSTALL json;
LOAD EXTENSION json;

CREATE NODE TABLE Person (id INT64, description JSON, primary key(id));
CREATE (p:Person {id: 20, description: to_json({height: 52, age: 32, scores: [1,2,5]})});
CREATE (p:Person {id: 40, description: to_json({age: 55, scores: [1,32,5,null], name: 'dan'})});
MATCH (p:Person) RETURN p.*;

Result:

┌───────┬────────────────────────────────────────────────┐
│ p.id  │ p.description                                  │
│ INT64 │ json                                           │
├───────┼────────────────────────────────────────────────┤
│ 20    │ {"height":52,"age":32,"scores":[1,2,5]}        │
│ 40    │ {"age":55,"scores":[1,32,5,null],"name":"dan"} │
└───────┴────────────────────────────────────────────────┘

You can then query on these properties as follows:

MATCH (p:Person)
WHERE json_extract(p.description, 'age') < 50
RETURN p.id AS id, json_extract(p.description, 'age') AS age;

Result:

┌───────┬──────┐
│ id    │ age  │
│ INT64 │ json │
├───────┼──────┤
│ 20    │ 32   │
└───────┴──────┘

JSON functions
This section lists the built-in functions that operate on the JSON data type within Kùzu.

to_json
Signature: ANY -> JSON

to_json(any)

Converts any Kùzu value to JSON.

Example 1:

RETURN to_json('{"name": "Gregory"}') AS person;

┌────────────────────┐
│ person             │
│ json               │
├────────────────────┤
│ {"name":"Gregory"} │
└────────────────────┘

Example 2:

RETURN to_json([1,2,3]) AS json_array;

┌────────────┐
│ json_array │
│ json       │
├────────────┤
│ [1,2,3]    │
└────────────┘

Example 3:

RETURN to_json('Alicia') AS simple_string;

┌───────────────┐
│ simple_string │
│ json          │
├───────────────┤
│ "Alicia"      │
└───────────────┘

array_to_json
Signature: ARRAY -> JSON

Alias for to_json that only accepts ARRAY.

row_to_json
Signature: LIST -> JSON

Alias for to_json that only accepts LIST.

cast(ANY AS JSON)
Signature: ANY -> JSON

Syntax sugar for to_json(any) -> JSON, cast can cast ANY type to be some type other than JSON as well, Read the instruction of Casting for more details.

Example:

RETURN cast('{"name": "Alicia", "age": 28}' AS JSON);

┌───────────────────────────────────────────┐
│ CAST({"name": "Alicia", "age": 28}, json) │
│ json                                      │
├───────────────────────────────────────────┤
│ {"name":"Alicia","age":28}                │
└───────────────────────────────────────────┘

json_object
Signature: STRING, ANY -> JSON object

json_object([key, value, ...])

Create a JSON object from any number of key, value pairs.

Example 1:

RETURN json_object("name", "Alicia");

┌──────────────────────────┐
│ json_object(name,Alicia) │
│ json                     │
├──────────────────────────┤
│ {"name":"Alicia"}        │
└──────────────────────────┘

Example 2:

RETURN json_object("name", "Alicia", "age", 28);

┌─────────────────────────────────┐
│ json_object(name,Alicia,age,28) │
│ json                            │
├─────────────────────────────────┤
│ {"name":"Alicia","age":28}      │
└─────────────────────────────────┘

json_array
Signature: ARRAY -> JSON ARRAY

json_array([any, ...])

Create an array of JSON objects from any number of values. Each value is converted into a JSON object.

Example:

RETURN json_array("Alicia", "25", NULL);

┌────────────────────────┐
│ json_array(Alicia,25,) │
│ json                   │
├────────────────────────┤
│ ["Alicia","25",null]   │
└────────────────────────┘

json_merge_patch
Signature: JSON, JSON -> JSON

json_merge_patch(json, json)

Merges TWO JSON documents. Applies RFC 7386

Example 1:

RETURN json_merge_patch('{"name": "Alicia"}', '{"age": 28}');

┌──────────────────────────────────────────────────┐
│ json_merge_patch({"name": "Alicia"},{"age": 28}) │
│ json                                             │
├──────────────────────────────────────────────────┤
│ {"name":"Alicia","age":28}                       │
└──────────────────────────────────────────────────┘

Example 2:

Merging with a NULL path would result in NULL path.

RETURN json_merge_patch("3", NULL);

┌──────────────────────┐
│ json_merge_patch(3,) │
│ json                 │
├──────────────────────┤
│                      │
└──────────────────────┘

json_extract
Signatures: JSON, STRING -> JSON, JSON, INTEGER -> JSON, JSON, LIST -> LIST of JSON

json_extract(json, path)

Extracts JSON from json at the given path. path is a STRING delimited by '/'. Integers may also be used to represent the path, which represents the index if the input is a JSON array. If path is a LIST of paths, the result will also be a LIST.

If the path does not exist, this function returns an empty JSON document.

Examples:

In this case, we provide a single item as the path.

RETURN json_extract('{"Software": {"Database": ["duck", "kuzu"]}}', 'Software/Database/1') AS extracted;

┌───────────┐
│ extracted │
│ json      │
├───────────┤
│ "kuzu"    │
└───────────┘

Here, we provide a LIST of paths:

RETURN json_extract('{"Software": {"Database": ["duck", "kuzu"]}}', ['Software/Database/1', 'Software/Database/0']) AS extracted;

┌─────────────────┐
│ extracted       │
│ json[]          │
├─────────────────┤
│ ["kuzu","duck"] │
└─────────────────┘

This example provides the path as an integer that represents the index of the item in the JSON array we want to extract:

RETURN json_extract('[1, 2, 42]', 2) AS nums;

┌──────┐
│ nums │
│ json │
├──────┤
│ 42   │
└──────┘

Extracting from an empty path results in empty JSON document:

RETURN json_extract('{"Software": {"Database": ["duck", "kuzu"]}}', "") AS extracted;

┌───────────┐
│ extracted │
│ json      │
├───────────┤
│           │
└───────────┘

json_array_length
Signature: JSON -> UINT32

json_array_length(json[])

If the json is an JSON array, return its length. Otherwise return 0.

Example 1:

RETURN json_array_length('["1", "1", "4", null]') AS len;

┌────────┐
│ len    │
│ UINT32 │
├────────┤
│ 4      │
└────────┘

Example 2:

Trying to compute the length of a JSON array with a null value results in a length of zero, as this isn’t valid JSON array.

RETURN json_array_length('{"kuzu": ["1", "1", "4", null]}') AS len;

┌────────┐
│ len    │
│ UINT32 │
├────────┤
│ 0      │
└────────┘

json_contains
Signature: JSON, JSON -> BOOL

json_contains(json_haystack, json_needle)

Returns True if json_needle is contained in json_haystack. Both parameters are of JSON type, but json_needle can also be a numeric value or a string, however the string must be wrapped in double quotes.

Example 1:

RETURN JSON_CONTAINS('{"name": "Alicia"}', '"Alicia"') AS found_name;

┌────────────┐
│ found_name │
│ BOOL       │
├────────────┤
│ True       │
└────────────┘

Example 2:

RETURN JSON_CONTAINS('{"age": 28}', '28') AS found_age;

┌───────────┐
│ found_age │
│ BOOL      │
├───────────┤
│ True      │
└───────────┘

json_keys
Signature: JSON -> STRING[]

json_keys(json)

Return keys of the root json object. If the root is not an JSON object, return an empty list.

Examples:

RETURN json_keys('{ "family": "anatidae", "species": [ "duck", "goose", "swan", null ] }') AS keys;

┌──────────────────┐
│ keys             │
│ STRING[]         │
├──────────────────┤
│ [family,species] │
└──────────────────┘

json_structure
Signature: JSON -> STRING

json_structure(json)

Returns the structure of the json in Kùzu type notation. Integer widths are automatically inferred from input values.

Example:

RETURN json_structure('[{"a": -1, "b": [1000, 2000, 3000]}, {"a": 2, "c": "hi"}]') AS structure;

┌─────────────────────────────────────────┐
│ structure                               │
│ STRING                                  │
├─────────────────────────────────────────┤
│ STRUCT(a INT16, b UINT16[], c STRING)[] │
└─────────────────────────────────────────┘

json_valid
Signature: JSON -> BOOL

json_valid(json)

Determines whether or not the provided json is valid JSON.

Example 1:

RETURN json_valid('{"name": "Alicia", "age": 28}') AS is_valid;

┌──────────┐
│ is_valid │
│ BOOL     │
├──────────┤
│ True     │
└──────────┘

Example 2:

RETURN json_valid('"name": "Alicia", "age": 28') AS is_valid;

┌──────────┐
│ is_valid │
│ BOOL     │
├──────────┤
│ False    │
└──────────┘

json
Signature: JSON -> JSON

json(json)

Parses and minifies the JSON.

Example:

UNWIND ['[        {"a":  [1],     "b": 2,"c": 1}, 1,    5, 9]', '[1, 2, 3]', '"ab"'] AS ARR RETURN json(ARR);

┌───────────────────────────────┐
│ json(ARR)                     │
│ json                          │
├───────────────────────────────┤
│ [{"a":[1],"b":2,"c":1},1,5,9] │
│ [1,2,3]                       │
│ "ab"                          │
└───────────────────────────────┘


