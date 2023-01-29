// Copyright 2016 TiKV Project Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package server

import "go.etcd.io/etcd/server/v3/embed"

// Config is the configuration for [Server]
type Config struct {
	// TODO
}

// GenEmbedEtcdConfig generates a configuration for embedded etcd.
func (c *Config) GenEmbedEtcdConfig() (*embed.Config, error) {
	// TODO
	return nil, nil
}
