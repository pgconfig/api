import { Button, Card, CardContent, Link } from "@momoi-labs/kiso-react";

import type { ComparisonParam } from "../lib/formatters.js";
import { useParameterDoc } from "../lib/useParameterDoc.js";
import { Icon } from "./Icon.js";
import { MarkdownContent } from "./MarkdownContent.js";

/**
 * What a parameter does and where to read more, shown under its row. The
 * PostgreSQL manual's entry replaces the text the API sent once it arrives.
 */
export function ComparisonRowDetail({
  row,
  pgVersion,
}: {
  row: ComparisonParam;
  pgVersion: string;
}) {
  const documentation = row.documentation ?? {};
  const readings = Object.entries(documentation.recomendations ?? {});
  const confUrl = `https://postgresqlco.nf/en/doc/param/${row.name}/${pgVersion}/`;
  const doc = useParameterDoc(pgVersion, row.name);
  const type = typeof doc?.fields.type === "string" ? doc.fields.type : documentation.type;
  const docsUrl = typeof doc?.fields.url === "string" ? doc.fields.url : documentation.url;

  return (
    <div className="comparison-detail">
      <div className="stack">
        <MarkdownContent source={documentation.abstract} breaks />
        {readings.length > 0 && (
          <div className="stack-xs">
            <p className="t-label">Suggested readings:</p>
            <ul className="comparison-readings">
              {readings.map(([description, url]) => (
                <li key={description}>
                  <Link href={url} target="_blank" rel="noreferrer">
                    {description}
                  </Link>
                </li>
              ))}
            </ul>
          </div>
        )}
      </div>

      <Card>
        <CardContent>
          <div>
            <strong className="mono">{row.name}</strong>
            <small className="muted">&nbsp;({type})</small>
          </div>
          {doc ? (
            <MarkdownContent source={doc.text} />
          ) : (
            documentation.details?.map((detail) => (
              <p key={detail} className="muted">
                {detail}
              </p>
            ))
          )}
          <div className="row-wrap">
            <Button asChild variant="primary">
              <a href={confUrl} target="_blank" rel="noreferrer">
                <Icon name="lightbulb-line" />
                <span>
                  Learn more on Postgresql<strong>co.nf</strong>
                </span>
              </a>
            </Button>
            {docsUrl && (
              <Button asChild>
                <a href={docsUrl} target="_blank" rel="noreferrer">
                  <Icon name="file-text-line" />
                  Check the docs
                </a>
              </Button>
            )}
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
