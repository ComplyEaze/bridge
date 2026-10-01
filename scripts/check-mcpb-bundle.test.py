# SPDX-License-Identifier: Apache-2.0
import importlib.util
import json
import os
from pathlib import Path
import sys
import tempfile
import unittest
import zipfile

spec = importlib.util.spec_from_file_location("mcpb_smoke", Path(__file__).with_name("check-mcpb-bundle.py"))
smoke = importlib.util.module_from_spec(spec)
spec.loader.exec_module(smoke)


class BundleSmokeTests(unittest.TestCase):
    def test_schema_receipts_require_matching_write_completion(self):
        response = b'{"jsonrpc":"2.0","id":3,"result":{}}\n'
        digest = smoke.hashlib.sha256(response).hexdigest()
        prepared = {"record_type": "response_prepared", "tool": "voucher_schema",
                    "receipt_id": "test-receipt", "bytes_prepared": len(response),
                    "response_sha256": digest}
        completed = {"record_type": "stdio_write_completed", "receipt_id": "test-receipt",
                     "bytes_written": len(response), "response_sha256": digest}
        def encoded(records):
            return b"\n".join(json.dumps(record).encode() for record in records) + b"\n"
        self.assertEqual(smoke.validate_schema_receipts(encoded([prepared, completed]), response), 2)
        with self.assertRaisesRegex(smoke.SmokeError, "schema_egress_completion_missing"):
            smoke.validate_schema_receipts(encoded([prepared]), response)
        for key, value in [("receipt_id", "other"), ("response_sha256", "wrong"),
                           ("bytes_written", 1), ("record_type", "response_prepared")]:
            with self.subTest(key=key), self.assertRaisesRegex(smoke.SmokeError, "schema_egress_completion_invalid"):
                smoke.validate_schema_receipts(encoded([prepared, dict(completed, **{key: value})]), response)

    def test_entry_point_cannot_escape_extraction_directory(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            archive = root / "invalid.mcpb"
            manifest = {"server": {"entry_point": "bin/../../outside"}}
            with zipfile.ZipFile(archive, "w") as bundle:
                bundle.writestr("manifest.json", json.dumps(manifest))
                for resource in smoke.RESOURCES:
                    bundle.writestr(resource, "packaging fixture")
                bundle.writestr(smoke.ICON, "packaging fixture")
                bundle.writestr("bin/bridge_mcp", "packaging fixture")
                bundle.writestr("bin/libpdfium.dylib", "packaging fixture")
                bundle.writestr(smoke.PDFIUM_NOTICE, "packaging fixture")
            with self.assertRaisesRegex(smoke.SmokeError, "invalid_archive_member_path"):
                smoke.unpack_bundle(archive, root / "unpacked", root)
            self.assertFalse((root / "outside").exists())
            self.assertFalse((root / "unpacked").exists())

    def pdfium_bundle(self, root, library_bytes=None, notice_bytes=None, include_library=True,
                      archive_icon=True, manifest_icon=None):
        """A bundle that passes every check before the PDFium ones, against a
        repository holding the real lock file."""
        repository = root / "repository"
        (repository / "packaging" / "pdfium").mkdir(parents=True)
        real = Path(__file__).resolve().parents[1]
        lock = (real / "packaging" / "pdfium" / "pdfium.lock.json").read_bytes()
        (repository / "packaging" / "pdfium" / "pdfium.lock.json").write_bytes(lock)
        for resource in smoke.RESOURCES:
            (repository / resource).write_text("packaging fixture")
        entry = "bin/host/bridge_mcp"
        library = "bin/host/" + smoke.PDFIUM_LIBRARIES[sys.platform]
        manifest = {"icon": manifest_icon or smoke.ICON,
                    "server": {"type": "binary", "entry_point": entry,
                               "mcp_config": {"command": "${__dirname}/" + entry}},
                    "compatibility": {"platforms": [sys.platform]}}
        icon = smoke.PNG_SIGNATURE + b"icon fixture"
        (repository / smoke.ICON_SOURCE).parent.mkdir(parents=True, exist_ok=True)
        (repository / smoke.ICON_SOURCE).write_bytes(icon)
        archive = root / "bundle.mcpb"
        with zipfile.ZipFile(archive, "w") as bundle:
            bundle.writestr("manifest.json", json.dumps(manifest))
            if archive_icon is not None:
                bundle.writestr(smoke.ICON, icon if archive_icon is True else archive_icon)
            for resource in smoke.RESOURCES:
                bundle.writestr(resource, "packaging fixture")
            info = zipfile.ZipInfo(entry)
            info.external_attr = 0o100755 << 16
            bundle.writestr(info, "binary fixture")
            bundle.writestr(library if include_library else "bin/host/other", library_bytes or b"wrong")
            bundle.writestr(smoke.PDFIUM_NOTICE, notice_bytes or b"wrong")
        return archive, repository

    @unittest.skipUnless(sys.platform in smoke.PDFIUM_LIBRARIES, "PDFium is bundled for macOS and Windows only")
    def test_bundled_pdfium_must_match_the_lock(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            archive, repository = self.pdfium_bundle(root)
            try:
                smoke.pdfium_pin(repository)
            except smoke.SmokeError:
                self.skipTest("this host architecture has no pinned PDFium")
            with self.assertRaisesRegex(smoke.SmokeError, "pdfium_library_not_pinned"):
                smoke.unpack_bundle(archive, root / "unpacked", repository)

    @unittest.skipUnless(sys.platform in smoke.PDFIUM_LIBRARIES, "PDFium is bundled for macOS and Windows only")
    def test_bundle_without_the_library_beside_the_binary_is_refused(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            archive, repository = self.pdfium_bundle(root, include_library=False)
            with self.assertRaisesRegex(smoke.SmokeError, "unexpected_archive_members"):
                smoke.unpack_bundle(archive, root / "unpacked", repository)

    @unittest.skipUnless(sys.platform in smoke.PDFIUM_LIBRARIES, "PDFium is bundled for macOS and Windows only")
    def test_the_icon_must_be_present_named_by_the_manifest_and_the_committed_file(self):
        cases = (
            ({"archive_icon": None}, "unexpected_archive_members"),
            ({"manifest_icon": "logo.png"}, "icon_declaration_mismatch"),
            ({"archive_icon": b"not a png"}, "icon_is_not_png"),
            ({"archive_icon": smoke.PNG_SIGNATURE + b"another image"}, "icon_bytes_differ"),
        )
        for options, code in cases:
            with self.subTest(code=code), tempfile.TemporaryDirectory() as temporary:
                root = Path(temporary)
                archive, repository = self.pdfium_bundle(root, **options)
                with self.assertRaisesRegex(smoke.SmokeError, code):
                    smoke.unpack_bundle(archive, root / "unpacked", repository)

    def test_the_shipped_notice_reserves_the_rights_in_the_icon(self):
        # NOTICE is copied into every archive and the smoke compares the archive's bytes to it, so
        # this is what a user of the package reads about the icon: not Apache-2.0, rights reserved.
        repository = Path(__file__).resolve().parents[1]
        text = " ".join((repository / "NOTICE").read_text(encoding="utf-8").split())
        self.assertIn("packaging/mcpb/icon.png", text)
        self.assertIn("are not licensed under Apache-2.0: all rights in them are reserved", text)
        self.assertIn("A fork or modified build must replace them", text)

    def test_the_committed_manifest_names_the_committed_icon(self):
        repository = Path(__file__).resolve().parents[1]
        manifest = json.loads((repository / "packaging/mcpb/manifest.json").read_text(encoding="utf-8"))
        self.assertEqual(manifest["icon"], smoke.ICON)
        self.assertTrue((repository / smoke.ICON_SOURCE).read_bytes().startswith(smoke.PNG_SIGNATURE))

    def test_an_unpinned_host_is_refused(self):
        repository = Path(__file__).resolve().parents[1]
        with self.assertRaisesRegex(smoke.SmokeError, "pdfium_platform_not_pinned"):
            smoke.pdfium_pin(repository, ("linux", "x86_64"))
        self.assertEqual(len(smoke.pdfium_pin(repository, ("darwin", "arm64"))["library_sha256"]), 64)
        self.assertEqual(len(smoke.pdfium_pin(repository, ("win32", "amd64"))["library_sha256"]), 64)

    def test_unsafe_member_names_are_rejected(self):
        for name in ("../outside", "/outside", "bin/../../outside", "bin\\outside", "bin/file:stream", "bin/\0file"):
            with self.subTest(name=name), self.assertRaises(smoke.SmokeError):
                smoke.member_path(name)

    def test_endpoint_settings_are_resolved_through_manifest_mappings(self):
        template = Path(__file__).resolve().parents[1] / "packaging/mcpb/manifest.json"
        manifest = json.loads(template.read_text(encoding="utf-8"))
        resolved = smoke.resolve_environment(manifest)
        self.assertEqual(resolved["BRIDGE_TALLY_HOST"], "127.0.0.1")
        self.assertEqual(resolved["BRIDGE_TALLY_PORT"], "9")
        self.assertEqual(resolved["BRIDGE_AGENT_ENABLE_WRITES"], "false")
        self.assertEqual(resolved["BRIDGE_AGENT_ENABLE_IMPORT"], "true")
        for key in ("BRIDGE_TALLY_HOST", "BRIDGE_TALLY_PORT"):
            broken = json.loads(template.read_text(encoding="utf-8"))
            broken["server"]["mcp_config"]["env"][key] = "${user_config.redaction}"
            with self.subTest(key=key), self.assertRaisesRegex(
                    smoke.SmokeError, "endpoint_environment_mapping_mismatch"):
                smoke.resolve_environment(broken)

    def test_initialize_server_version_must_match_the_archived_manifest(self):
        manifest = {"version": "0.2.0"}
        response = {"result": {"serverInfo": {"version": "0.2.0"}}}
        self.assertEqual(smoke.validate_server_version(response, manifest), "0.2.0")
        with self.assertRaisesRegex(smoke.SmokeError, "server_version_mismatch"):
            smoke.validate_server_version(
                {"result": {"serverInfo": {"version": "0.2.1"}}}, manifest)

    def test_posting_defaults_off_and_a_default_on_manifest_is_refused(self):
        template = Path(__file__).resolve().parents[1] / "packaging/mcpb/manifest.json"
        manifest = json.loads(template.read_text(encoding="utf-8"))
        self.assertIs(manifest["user_config"]["enable_writes"]["default"], False)
        manifest["user_config"]["enable_writes"]["default"] = True
        with self.assertRaisesRegex(smoke.SmokeError, "posting_must_default_off"):
            smoke.resolve_environment(manifest)

    def test_the_default_bundle_prepares_and_parses_but_does_not_post(self):
        template = Path(__file__).resolve().parents[1] / "packaging/mcpb/manifest.json"
        default = smoke.resolve_environment(json.loads(template.read_text(encoding="utf-8")))
        tools = smoke.expected_tools(default)
        self.assertTrue({"build_import_xml", "parse_bank_statement", "verify_import"} <= tools)
        self.assertNotIn("post_import", tools)
        self.assertNotIn("acknowledge_post_review", tools)
        enabled = smoke.expected_tools(dict(default, BRIDGE_AGENT_ENABLE_WRITES="true"))
        self.assertEqual(enabled - tools, {"post_import", "acknowledge_post_review"})

    def test_import_mapping_must_be_the_constant_true(self):
        template = Path(__file__).resolve().parents[1] / "packaging/mcpb/manifest.json"
        for mapping in ("false", "${user_config.enable_writes}", "1"):
            manifest = json.loads(template.read_text(encoding="utf-8"))
            manifest["server"]["mcp_config"]["env"]["BRIDGE_AGENT_ENABLE_IMPORT"] = mapping
            with self.subTest(mapping=mapping), self.assertRaisesRegex(
                    smoke.SmokeError, "import_environment_mapping_mismatch"):
                smoke.resolve_environment(manifest)
        manifest = json.loads(template.read_text(encoding="utf-8"))
        del manifest["server"]["mcp_config"]["env"]["BRIDGE_AGENT_ENABLE_IMPORT"]
        with self.assertRaisesRegex(smoke.SmokeError, "unexpected_environment_mapping"):
            smoke.resolve_environment(manifest)

    def test_write_setting_requires_a_real_boolean_default(self):
        template = Path(__file__).resolve().parents[1] / "packaging/mcpb/manifest.json"
        for default in ("true", "false", 0, 1, None):
            manifest = json.loads(template.read_text(encoding="utf-8"))
            manifest["user_config"]["enable_writes"]["default"] = default
            with self.subTest(default=default), self.assertRaisesRegex(
                    smoke.SmokeError, "writes_default_must_be_boolean"):
                smoke.resolve_environment(manifest)

    def test_write_mapping_cannot_bypass_user_setting(self):
        template = Path(__file__).resolve().parents[1] / "packaging/mcpb/manifest.json"
        for mapping in ("true", "false", "${user_config.redaction}"):
            manifest = json.loads(template.read_text(encoding="utf-8"))
            manifest["server"]["mcp_config"]["env"]["BRIDGE_AGENT_ENABLE_WRITES"] = mapping
            with self.subTest(mapping=mapping), self.assertRaisesRegex(
                    smoke.SmokeError, "writes_environment_mapping_mismatch"):
                smoke.resolve_environment(manifest)

    def test_terms_setting_defaults_to_not_accepted_and_is_mapped(self):
        template = Path(__file__).resolve().parents[1] / "packaging/mcpb/manifest.json"
        manifest = json.loads(template.read_text(encoding="utf-8"))
        self.assertEqual(smoke.resolve_environment(manifest)["BRIDGE_TERMS_ACCEPTED"], "false")
        for field, value in (("default", True), ("required", False), ("type", "string")):
            broken = json.loads(template.read_text(encoding="utf-8"))
            broken["user_config"]["accept_terms_2026_10"][field] = value
            with self.subTest(field=field), self.assertRaisesRegex(
                    smoke.SmokeError, "terms_setting_must_be_required_and_off"):
                smoke.resolve_environment(broken)
        broken = json.loads(template.read_text(encoding="utf-8"))
        del broken["user_config"]["accept_terms_2026_10"]
        with self.assertRaisesRegex(smoke.SmokeError, "terms_setting_must_be_required_and_off"):
            smoke.resolve_environment(broken)
        for mapping in ("true", "${user_config.enable_writes}"):
            broken = json.loads(template.read_text(encoding="utf-8"))
            broken["server"]["mcp_config"]["env"]["BRIDGE_TERMS_ACCEPTED"] = mapping
            with self.subTest(mapping=mapping), self.assertRaisesRegex(
                    smoke.SmokeError, "terms_environment_mapping_mismatch"):
                smoke.resolve_environment(broken)
        broken = json.loads(template.read_text(encoding="utf-8"))
        del broken["server"]["mcp_config"]["env"]["BRIDGE_TERMS_ACCEPTED"]
        with self.assertRaisesRegex(smoke.SmokeError, "unexpected_environment_mapping"):
            smoke.resolve_environment(broken)

    def test_server_output_is_bounded(self):
        command = [sys.executable, "-c", "import sys; sys.stdout.write('x' * 1048576); sys.stdout.flush()"]
        with self.assertRaisesRegex(smoke.SmokeError, "server_output_limit"):
            smoke.run_bounded(command, b"", os.environ.copy())

    def test_server_timeout_is_enforced(self):
        command = [sys.executable, "-c", "import time; time.sleep(10)"]
        with self.assertRaisesRegex(smoke.SmokeError, "server_timeout"):
            smoke.run_bounded(command, b"", os.environ.copy(), timeout=0.1)


if __name__ == "__main__":
    unittest.main()
