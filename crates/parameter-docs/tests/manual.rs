//! `config.sgml` read into parameters with Markdown text.

use pgconfig_parameter_docs::manual::{Book, ManualEntry, entries};

/// A chapter shaped like `config.sgml`, around the entries a test supplies.
fn config(entries: &str) -> String {
    format!(
        r#"<chapter id="runtime-config">
 <title>Server Configuration</title>
 <sect1 id="runtime-config-resource">
  <title>Resource Consumption</title>
  <sect2 id="runtime-config-resource-memory">
   <title>Memory</title>
   <variablelist>
{entries}
   </variablelist>
  </sect2>
 </sect1>
</chapter>
"#
    )
}

fn read(source: &str) -> Vec<ManualEntry> {
    let book = Book::index([("config.sgml", source)]).unwrap();
    entries(source, &book, "18").unwrap()
}

#[test]
fn a_parameter_has_its_name_type_url_and_text() {
    let source = config(
        r#"
     <varlistentry id="guc-work-mem" xreflabel="work_mem">
      <term><varname>work_mem</varname> (<type>integer</type>)
      <indexterm>
       <primary><varname>work_mem</varname> configuration parameter</primary>
      </indexterm>
      </term>
      <listitem>
       <para>
        Sets the base maximum amount of memory to be used by a query
        operation.  The default value is four megabytes (<literal>4MB</literal>).
       </para>
      </listitem>
     </varlistentry>"#,
    );

    assert_eq!(
        read(&source),
        [ManualEntry {
            name: "work_mem".into(),
            param_type: Some("integer".into()),
            url: "https://www.postgresql.org/docs/18/runtime-config-resource.html#GUC-WORK-MEM"
                .into(),
            text: "Sets the base maximum amount of memory to be used by a query operation. The default value is four megabytes (`4MB`).".into(),
        }]
    );
}

/// A chapter and a reference page that the entries point to.
const WAL: &str = r#"<chapter id="wal">
 <title>Reliability and the Write-Ahead Log</title>
 <sect1 id="wal-async-commit">
  <title>Asynchronous Commit</title>
  <sect2 id="wal-commit-delay"><title>Commit Delay</title><para>Text.</para></sect2>
 </sect1>
</chapter>
"#;

const ALTER_SYSTEM: &str = r#"<refentry id="sql-altersystem">
 <refmeta><refentrytitle>ALTER SYSTEM</refentrytitle><manvolnum>7</manvolnum></refmeta>
 <refnamediv><refname>ALTER SYSTEM</refname></refnamediv>
</refentry>
"#;

#[test]
fn cross_references_become_links_into_the_manual_of_the_same_version() {
    let source = config(
        r#"
     <varlistentry id="guc-hash-mem-multiplier" xreflabel="hash_mem_multiplier">
      <term><varname>hash_mem_multiplier</varname> (<type>floating point</type>)</term>
      <listitem><para>Multiplies work_mem.</para></listitem>
     </varlistentry>
     <varlistentry id="guc-work-mem" xreflabel="work_mem">
      <term><varname>work_mem</varname> (<type>integer</type>)</term>
      <listitem>
       <para>
        Hash tables use <xref linkend="guc-hash-mem-multiplier"/> times this value.
       </para>
       <para>
        See <xref linkend="wal-async-commit"/>, <xref linkend="sql-altersystem"/>,
        <link linkend="wal-commit-delay">the commit delay</link> and
        <ulink url="https://wiki.postgresql.org/wiki/Tuning">the wiki</ulink>.
       </para>
      </listitem>
     </varlistentry>"#,
    );
    let book = Book::index([
        ("config.sgml", source.as_str()),
        ("wal.sgml", WAL),
        ("ref/alter_system.sgml", ALTER_SYSTEM),
    ])
    .unwrap();

    let work_mem = entries(&source, &book, "18").unwrap().remove(1);

    assert_eq!(
        work_mem.text,
        "Hash tables use [`hash_mem_multiplier`](https://www.postgresql.org/docs/18/runtime-config-resource.html#GUC-HASH-MEM-MULTIPLIER) times this value.\n\n\
         See [Asynchronous Commit](https://www.postgresql.org/docs/18/wal-async-commit.html), \
         [ALTER SYSTEM](https://www.postgresql.org/docs/18/sql-altersystem.html), \
         [the commit delay](https://www.postgresql.org/docs/18/wal-async-commit.html#WAL-COMMIT-DELAY) \
         and [the wiki](https://wiki.postgresql.org/wiki/Tuning)."
    );
}

#[test]
fn sgml_before_postgresql_11_reads_like_xml() {
    // `</>` closes the open element, an `xref` has no end tag, and an
    // attribute value may go unquoted.
    let source = config(
        r#"
     <varlistentry id="guc-maintenance-work-mem" xreflabel="maintenance_work_mem">
      <term><varname>maintenance_work_mem</varname> (<type>integer</type>)</term>
      <listitem><para>For maintenance.</para></listitem>
     </varlistentry>
     <varlistentry id="guc-work-mem" xreflabel="work_mem">
      <term><varname>work_mem</varname> (<type>integer</type>)</term>
      <indexterm>
       <primary><varname>work_mem</> configuration parameter</primary>
      </indexterm>
      <listitem>
       <para>
        The value defaults to one megabyte (<literal>1MB</>).
        See <xref linkend=guc-maintenance-work-mem>.
       </para>
      </listitem>
     </varlistentry>"#,
    );
    let book = Book::index([("config.sgml", source.as_str())]).unwrap();

    let work_mem = entries(&source, &book, "9.1").unwrap().remove(1);

    assert_eq!(
        work_mem.text,
        "The value defaults to one megabyte (`1MB`). See [`maintenance_work_mem`](https://www.postgresql.org/docs/9.1/runtime-config-resource.html#GUC-MAINTENANCE-WORK-MEM)."
    );
}

#[test]
fn an_entry_with_several_terms_documents_each_parameter() {
    let source = config(
        r#"
     <varlistentry id="guc-debug-print-parse">
      <term><varname>debug_print_parse</varname> (<type>boolean</type>)</term>
      <term><varname>debug_print_plan</varname> (<type>boolean</type>)</term>
      <listitem><para>Print trees.</para></listitem>
     </varlistentry>"#,
    );

    let names: Vec<(String, String)> = read(&source)
        .into_iter()
        .map(|entry| (entry.name, entry.url))
        .collect();

    let url =
        "https://www.postgresql.org/docs/18/runtime-config-resource.html#GUC-DEBUG-PRINT-PARSE";
    assert_eq!(
        names,
        [
            ("debug_print_parse".to_string(), url.to_string()),
            ("debug_print_plan".to_string(), url.to_string()),
        ]
    );
}

#[test]
fn a_list_of_values_inside_a_description_is_part_of_the_text() {
    let source = config(
        r#"
     <varlistentry id="guc-ssl-ciphers" xreflabel="ssl_ciphers">
      <term><varname>ssl_ciphers</varname> (<type>string</type>)</term>
      <listitem>
       <para>The default is:</para>
       <variablelist>
        <varlistentry id="guc-ssl-ciphers-high">
         <term><literal>HIGH</literal></term>
         <listitem><para>Cipher suites that use ciphers from the high group.</para></listitem>
        </varlistentry>
        <varlistentry>
         <term><literal>!aNULL</literal></term>
         <listitem>
          <para>Disables anonymous cipher suites.</para>
          <para>They are unsafe.</para>
         </listitem>
        </varlistentry>
       </variablelist>
      </listitem>
     </varlistentry>"#,
    );

    let entries = read(&source);

    assert_eq!(entries.len(), 1);
    assert_eq!(
        entries[0].text,
        "The default is:\n\n\
         - `HIGH`: Cipher suites that use ciphers from the high group.\n\n\
         - `!aNULL`: Disables anonymous cipher suites.\n\n  They are unsafe."
    );
}

/// The text of one parameter whose description is `body`.
fn text(body: &str) -> String {
    let source = config(&format!(
        r#"
     <varlistentry id="guc-example" xreflabel="example">
      <term><varname>example</varname> (<type>string</type>)</term>
      <listitem>{body}</listitem>
     </varlistentry>"#
    ));
    read(&source).remove(0).text
}

#[test]
fn a_paragraph_can_hold_a_list() {
    let body = r#"
       <para>
        Valid values are:
        <itemizedlist>
         <listitem><para><literal>off</literal></para></listitem>
         <listitem><para><literal>on</literal></para></listitem>
        </itemizedlist>
        and, in order of preference:
        <orderedlist>
         <listitem><para>first</para></listitem>
         <listitem><para>second</para></listitem>
        </orderedlist>
       </para>"#;

    assert_eq!(
        text(body),
        "Valid values are:\n\n- `off`\n- `on`\n\nand, in order of preference:\n\n1. first\n2. second"
    );
}

#[test]
fn notes_tips_and_warnings_become_alerts() {
    let body = r#"
       <para>Before.</para>
       <note>
        <para>One.</para>
        <para>Two.</para>
       </note>
       <tip><para>A tip.</para></tip>
       <warning><para>A warning.</para></warning>
       <caution><para>A caution.</para></caution>
       <important><para>Important.</para></important>"#;

    assert_eq!(
        text(body),
        "Before.\n\n\
         > [!NOTE]\n> One.\n>\n> Two.\n\n\
         > [!TIP]\n> A tip.\n\n\
         > [!WARNING]\n> A warning.\n\n\
         > [!CAUTION]\n> A caution.\n\n\
         > [!IMPORTANT]\n> Important."
    );
}

#[test]
fn listings_keep_their_lines_in_code_blocks() {
    let body = "
       <para>For example:</para>
<programlisting>
SET work_mem = '64MB';
SELECT <replaceable>column</replaceable> FROM t;
</programlisting>
<synopsis>
host    all    all    0.0.0.0/0    md5
</synopsis>";

    assert_eq!(
        text(body),
        "For example:\n\n```\nSET work_mem = '64MB';\nSELECT column FROM t;\n```\n\n```\nhost    all    all    0.0.0.0/0    md5\n```"
    );
}

#[test]
fn tables_become_markdown_tables() {
    let body = r#"
       <para>The modes:</para>
       <table id="synchronous-commit-matrix">
        <title>synchronous_commit Modes</title>
        <tgroup cols="3">
         <colspec colname="col1" colwidth="1.5*"/>
         <thead>
          <row><entry>setting</entry><entry>local durable</entry><entry>remote | apply</entry></row>
         </thead>
         <tbody>
          <row><entry><literal>on</literal></entry><entry align="center">&bull;</entry><entry></entry></row>
         </tbody>
        </tgroup>
       </table>
       <informaltable>
        <tgroup cols="2">
         <tbody>
          <row><entry>a</entry><entry><para>b</para></entry></row>
          <row><entry>c</entry><entry>d</entry></row>
         </tbody>
        </tgroup>
       </informaltable>"#;

    assert_eq!(
        text(body),
        "The modes:\n\n\
         **synchronous_commit Modes**\n\n\
         | setting | local durable | remote \\| apply |\n| --- | --- | --- |\n| `on` | \u{2022} |  |\n\n\
         | a | b |\n| --- | --- |\n| c | d |"
    );
}

#[test]
fn inline_markup_keeps_its_meaning_and_text_is_escaped() {
    let body = r#"
       <para>
        The <firstterm>planner</firstterm> uses <emphasis>all</emphasis> of
        <application>PostgreSQL</application>'s <quote>knowledge</quote>; run
        <command>VACUUM</command> on <filename>pg_hba.conf</filename> with
        <option>-c</option> or <replaceable>name</replaceable>,
        2<superscript>31</superscript> - 1 = <literal>listen_addresses = '*'</literal>
        *not* [all] _x &lt;b&gt; \ and
        <citerefentry><refentrytitle>dlopen</refentrytitle><manvolnum>3</manvolnum></citerefentry>.
       </para>"#;

    assert_eq!(
        text(body),
        "The *planner* uses *all* of PostgreSQL's \"knowledge\"; run `VACUUM` on `pg_hba.conf` \
         with `-c` or *name*, 2^31 - 1 = `listen_addresses = '*'` \\*not\\* \\[all\\] \\_x \\<b> \\\\ \
         and dlopen(3)."
    );
}

#[test]
fn markup_the_renderer_does_not_know_stops_the_extraction() {
    let source = config(
        r#"
     <varlistentry id="guc-example" xreflabel="example">
      <term><varname>example</varname></term>
      <listitem><para>A <blink>new</blink> element.</para></listitem>
     </varlistentry>"#,
    );
    let book = Book::index([("config.sgml", source.as_str())]).unwrap();

    let error = entries(&source, &book, "18").unwrap_err();

    assert_eq!(error, "guc-example: <blink> is not rendered");
}

#[test]
fn a_simple_list_is_a_bullet_list_unless_it_is_inline() {
    let body = r#"
       <para>
        Operations include:
        <simplelist>
         <member><command>CLUSTER</command></member>
         <member><command>COPY</command> into new tables</member>
        </simplelist>
        and also <simplelist type="inline"><member>a</member><member>b</member></simplelist>.
       </para>"#;

    assert_eq!(
        text(body),
        "Operations include:\n\n- `CLUSTER`\n- `COPY` into new tables\n\nand also a, b."
    );
}

#[test]
fn a_parameter_documented_in_two_places_reads_as_both() {
    let source = config(
        r#"
     <varlistentry id="guc-max-replication-slots" xreflabel="max_replication_slots">
      <term><varname>max_replication_slots</varname> (<type>integer</type>)</term>
      <listitem><para>On a sending server.</para></listitem>
     </varlistentry>
     <varlistentry id="guc-max-replication-slots-subscriber" xreflabel="max_replication_slots">
      <term><varname>max_replication_slots</varname> (<type>integer</type>)</term>
      <listitem><para>On a subscriber.</para></listitem>
     </varlistentry>"#,
    );

    let entries = read(&source);

    assert_eq!(entries.len(), 1);
    assert_eq!(
        entries[0].url,
        "https://www.postgresql.org/docs/18/runtime-config-resource.html#GUC-MAX-REPLICATION-SLOTS"
    );
    assert_eq!(entries[0].text, "On a sending server.\n\nOn a subscriber.");
}
