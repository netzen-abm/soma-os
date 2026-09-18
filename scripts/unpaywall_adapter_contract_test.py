"""Focused Unpaywall adapter contract tests."""

from pathlib import Path
import sys

ADAPTER_DIR = Path(__file__).resolve().parent.parent / "services" / "evidence-research" / "adapters"
sys.path.insert(0, str(ADAPTER_DIR))

from unpaywall_adapter import UnpaywallAdapter, UnpaywallProviderError


def test_source_contract_is_provider_scoped() -> None:
    source = UnpaywallAdapter("test@example.invalid").source_contract()
    source.validate()
    assert source.source_id == "unpaywall"


def test_resolve_preserves_access_location_provenance() -> None:
    adapter = UnpaywallAdapter("test@example.invalid")
    adapter._request_payload = None

    class Response:
        def read(self):
            return b'{"doi":"10.1234/example","oa_locations":[{"url":"https://repository.example/a.pdf","host_type":"repository","oa_version":"publishedVersion","license":"cc-by"}]}'
        def __enter__(self): return self
        def __exit__(self, *args): pass

    import urllib.request
    original = urllib.request.urlopen
    urllib.request.urlopen = lambda *args, **kwargs: Response()
    try:
        locations = adapter.resolve("10.1234/example")
    finally:
        urllib.request.urlopen = original

    assert len(locations) == 1
    assert locations[0].provider_id == "unpaywall"
    assert locations[0].work_identifier == "10.1234/example"
    assert locations[0].url.endswith("a.pdf")
    assert locations[0].is_open_access is True
    assert locations[0].source is not None


def test_provider_failure_is_not_empty_success() -> None:
    adapter = UnpaywallAdapter("test@example.invalid")

    import urllib.request
    original = urllib.request.urlopen
    def fail(*args, **kwargs):
        raise OSError("timeout")
    urllib.request.urlopen = fail
    try:
        try:
            adapter.resolve("10.1234/example")
        except UnpaywallProviderError:
            pass
        else:
            raise AssertionError("provider failure must remain explicit")
    finally:
        urllib.request.urlopen = original
