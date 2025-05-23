pkgdatadir = get_option('prefix') / get_option('datadir') / meson.project_name()
gnome = import('gnome')

gnome.compile_resources(
  'mailviewer',
  'mailviewer.gresource.xml',
  gresource_bundle: true,
  install: true,
  install_dir: pkgdatadir,
)

conf = configuration_data()
conf.set_quoted('APP_ID', application_id)
conf.set_quoted('APP_NAME', meson.project_name())
conf.set_quoted('VERSION', meson.project_version())
conf.set_quoted('GETTEXT_PACKAGE', 'mailviewer')
conf.set_quoted('LOCALEDIR', get_option('prefix') / get_option('localedir'))
conf.set_quoted('PKGDATADIR', pkgdatadir)

configure_file(input: 'config.rs.in', output: 'config.rs', configuration: conf)

# Copy the config.rs output to the source directory.
run_command(
  'cp',
  meson.project_build_root() / 'src' / 'config.rs',
  meson.project_source_root() / 'src' / 'config.rs',
  check: true,
)

cargo_bin = find_program('cargo')
cargo_opt = ['--manifest-path', meson.project_source_root() / 'Cargo.toml']
cargo_opt += ['--target-dir', meson.project_build_root() / 'src']
cargo_env = ['CARGO_HOME=' + meson.project_build_root() / 'cargo-home', 'MESON_BUILD=1']

if get_option('buildtype') == 'release'
  cargo_opt += ['--release']
  rust_target = 'release'
else
  rust_target = 'debug'
endif

cmd = run_command('sh', '-c', 'echo $OFFLINE_BUILD')
OFFLINE_BUILD = cmd.stdout().strip()
message('OFFLINE_BUILD=' + OFFLINE_BUILD)

if OFFLINE_BUILD == '1'
  message('Offline build options')
  cargo_opt += ['--offline']
  cargo_env = ['CARGO_HOME=' + '/run/build/mailviewer/cargo']
endif

message('OFFLINE_BUILD=' + OFFLINE_BUILD)
message('rust_target=' + rust_target)
message('buildtype=' + get_option('buildtype'))

message('cargo_opt=')
message(cargo_opt)

custom_target(
  'cargo-build',
  build_by_default: true,
  build_always_stale: true,
  output: meson.project_name(),
  console: true,
  install: true,
  install_dir: get_option('bindir'),
  command: [
    'env',
    cargo_env,
    cargo_bin,
    'build',
    cargo_opt,
    '&&',
    'cp',
    'src' / rust_target / meson.project_name(),
    '@OUTPUT@',
  ],
)