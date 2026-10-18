Embeddable TAK Server for [cottak](https://crates.io/crates/cottak).

Accepts TAK clients over plain TCP and mutual TLS, speaks XML and TAK Protocol
v1 (with negotiation), answers pings, and routes events between clients
(broadcast, `<marti><dest callsign=...>` or `<dest mission=...>`), replaying the latest
positions to new clients. It also mocks the Marti HTTPS API (missions, files, contacts,
subscriptions), so the `cottak-marti` client can be tested end to end. Intended for integration
tests, simulators and small isolated networks. It has no user accounts, groups, persistence or
federation, so it is not a replacement for the official TAK Server.
