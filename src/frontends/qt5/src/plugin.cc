#include "plugin.hpp"
#include "input_context.hpp"
#include <QtWidgets/QApplication>
#include <stdexcept>

KimePlatformInputContextPlugin::KimePlatformInputContextPlugin() {
  if (kime::kime_api_version() != kime::KIME_API_VERSION) {
    throw std::runtime_error("Kime Engine version is mismatched!");
  }

  this->config = kime::kime_config_load();
  if (!this->config) {
    throw std::runtime_error("Failed to load kime config");
  }
  this->engine = kime::kime_engine_new(this->config);
}

KimePlatformInputContextPlugin::~KimePlatformInputContextPlugin() {
  kime::kime_engine_delete(this->engine);
  kime::kime_config_delete(this->config);
}

QPlatformInputContext *
KimePlatformInputContextPlugin::create(const QString &key,
                                       const QStringList &param_list) {
  return new KimeInputContext(this->engine, this->config);
}
