
Syntax
seed
agent Researcher {
    fn research(query: string) -> ResearchResult {
        let findings = perform search(query);
        discharge findings with { confidence: 0.85 } {
            synthesize(findings)
        }
    }
}
Top-level items: agent, section, fn, struct, enum, mod, use
