import assert from "node:assert/strict";
import { it } from "node:test";
import {
  createComparisonStructure as structure,
  formatConfigs,
} from "../src/lib/formatters.ts";

const response = {
  data: [
    {
      configuration: [
        {
          category: "memory_related",
          description: "Memory Configuration",
          parameters: [
            {
              config_value: "512MB",
              format: "Bytes",
              name: "shared_buffers",
            },
            {
              config_value: "2GB",
              format: "Bytes",
              name: "effective_cache_size",
            },
            { config_value: "20MB", format: "Bytes", name: "work_mem" },
            {
              config_value: "128MB",
              format: "Bytes",
              name: "maintenance_work_mem",
            },
          ],
        },
        {
          category: "checkpoint_related",
          description: "Checkpoint Related Configuration",
          parameters: [
            {
              config_value: "512MB",
              format: "Bytes",
              name: "min_wal_size",
            },
            {
              config_value: "2GB",
              format: "Bytes",
              name: "max_wal_size",
            },
            {
              config_value: 0.7,
              format: "Float",
              name: "checkpoint_completion_target",
            },
            { config_value: "15MB", format: "Bytes", name: "wal_buffers" },
          ],
        },
        {
          category: "network_related",
          description: "Network Related Configuration",
          parameters: [
            {
              config_value: "*",
              format: "String",
              name: "listen_addresses",
            },
            {
              config_value: 100,
              format: "Decimal",
              name: "max_connections",
            },
          ],
        },
        {
          category: "hard_drive_type",
          description: "Hard Drive Configuration",
          parameters: [
            {
              config_value: 1.1,
              format: "Float",
              name: "random_page_cost",
            },
            {
              config_value: 200,
              format: "Decimal",
              name: "effective_io_concurrency",
            },
          ],
        },
        {
          category: "worker_processes",
          description: "Worker Processes",
          parameters: [
            {
              config_value: 1,
              format: "Decimal",
              name: "max_worker_processes",
            },
            {
              config_value: 2,
              format: "Decimal",
              name: "max_parallel_workers_per_gather",
            },
            {
              config_value: 1,
              format: "Decimal",
              name: "max_parallel_workers",
            },
          ],
        },
      ],
      environment: "WEB",
    },
    {
      configuration: [
        {
          category: "memory_related",
          description: "Memory Configuration",
          parameters: [
            {
              config_value: "512MB",
              format: "Bytes",
              name: "shared_buffers",
            },
            {
              config_value: "2GB",
              format: "Bytes",
              name: "effective_cache_size",
            },
            { config_value: "20MB", format: "Bytes", name: "work_mem" },
            {
              config_value: "128MB",
              format: "Bytes",
              name: "maintenance_work_mem",
            },
          ],
        },
        {
          category: "checkpoint_related",
          description: "Checkpoint Related Configuration",
          parameters: [
            {
              config_value: "1GB",
              format: "Bytes",
              name: "min_wal_size",
            },
            {
              config_value: "3GB",
              format: "Bytes",
              name: "max_wal_size",
            },
            {
              config_value: 0.9,
              format: "Float",
              name: "checkpoint_completion_target",
            },
            { config_value: "15MB", format: "Bytes", name: "wal_buffers" },
          ],
        },
        {
          category: "network_related",
          description: "Network Related Configuration",
          parameters: [
            {
              config_value: "*",
              format: "String",
              name: "listen_addresses",
            },
            {
              config_value: 100,
              format: "Decimal",
              name: "max_connections",
            },
          ],
        },
        {
          category: "hard_drive_type",
          description: "Hard Drive Configuration",
          parameters: [
            {
              config_value: 1.1,
              format: "Float",
              name: "random_page_cost",
            },
            {
              config_value: 200,
              format: "Decimal",
              name: "effective_io_concurrency",
            },
          ],
        },
        {
          category: "worker_processes",
          description: "Worker Processes",
          parameters: [
            {
              config_value: 1,
              format: "Decimal",
              name: "max_worker_processes",
            },
            {
              config_value: 2,
              format: "Decimal",
              name: "max_parallel_workers_per_gather",
            },
            {
              config_value: 1,
              format: "Decimal",
              name: "max_parallel_workers",
            },
          ],
        },
      ],
      environment: "OLTP",
    },
    {
      configuration: [
        {
          category: "memory_related",
          description: "Memory Configuration",
          parameters: [
            {
              config_value: "512MB",
              format: "Bytes",
              name: "shared_buffers",
            },
            {
              config_value: "2GB",
              format: "Bytes",
              name: "effective_cache_size",
            },
            { config_value: "10MB", format: "Bytes", name: "work_mem" },
            {
              config_value: "256MB",
              format: "Bytes",
              name: "maintenance_work_mem",
            },
          ],
        },
        {
          category: "checkpoint_related",
          description: "Checkpoint Related Configuration",
          parameters: [
            {
              config_value: "2GB",
              format: "Bytes",
              name: "min_wal_size",
            },
            {
              config_value: "6GB",
              format: "Bytes",
              name: "max_wal_size",
            },
            {
              config_value: 0.9,
              format: "Float",
              name: "checkpoint_completion_target",
            },
            { config_value: "15MB", format: "Bytes", name: "wal_buffers" },
          ],
        },
        {
          category: "network_related",
          description: "Network Related Configuration",
          parameters: [
            {
              config_value: "*",
              format: "String",
              name: "listen_addresses",
            },
            {
              config_value: 100,
              format: "Decimal",
              name: "max_connections",
            },
          ],
        },
        {
          category: "hard_drive_type",
          description: "Hard Drive Configuration",
          parameters: [
            {
              config_value: 1.1,
              format: "Float",
              name: "random_page_cost",
            },
            {
              config_value: 200,
              format: "Decimal",
              name: "effective_io_concurrency",
            },
          ],
        },
        {
          category: "worker_processes",
          description: "Worker Processes",
          parameters: [
            {
              config_value: 1,
              format: "Decimal",
              name: "max_worker_processes",
            },
            {
              config_value: 2,
              format: "Decimal",
              name: "max_parallel_workers_per_gather",
            },
            {
              config_value: 1,
              format: "Decimal",
              name: "max_parallel_workers",
            },
          ],
        },
      ],
      environment: "DW",
    },
    {
      configuration: [
        {
          category: "memory_related",
          description: "Memory Configuration",
          parameters: [
            {
              config_value: "512MB",
              format: "Bytes",
              name: "shared_buffers",
            },
            {
              config_value: "2GB",
              format: "Bytes",
              name: "effective_cache_size",
            },
            { config_value: "10MB", format: "Bytes", name: "work_mem" },
            {
              config_value: "128MB",
              format: "Bytes",
              name: "maintenance_work_mem",
            },
          ],
        },
        {
          category: "checkpoint_related",
          description: "Checkpoint Related Configuration",
          parameters: [
            {
              config_value: "512MB",
              format: "Bytes",
              name: "min_wal_size",
            },
            {
              config_value: "2GB",
              format: "Bytes",
              name: "max_wal_size",
            },
            {
              config_value: 0.9,
              format: "Float",
              name: "checkpoint_completion_target",
            },
            { config_value: "15MB", format: "Bytes", name: "wal_buffers" },
          ],
        },
        {
          category: "network_related",
          description: "Network Related Configuration",
          parameters: [
            {
              config_value: "*",
              format: "String",
              name: "listen_addresses",
            },
            {
              config_value: 100,
              format: "Decimal",
              name: "max_connections",
            },
          ],
        },
        {
          category: "hard_drive_type",
          description: "Hard Drive Configuration",
          parameters: [
            {
              config_value: 1.1,
              format: "Float",
              name: "random_page_cost",
            },
            {
              config_value: 200,
              format: "Decimal",
              name: "effective_io_concurrency",
            },
          ],
        },
        {
          category: "worker_processes",
          description: "Worker Processes",
          parameters: [
            {
              config_value: 1,
              format: "Decimal",
              name: "max_worker_processes",
            },
            {
              config_value: 2,
              format: "Decimal",
              name: "max_parallel_workers_per_gather",
            },
            {
              config_value: 1,
              format: "Decimal",
              name: "max_parallel_workers",
            },
          ],
        },
      ],
      environment: "Mixed",
    },
    {
      configuration: [
        {
          category: "memory_related",
          description: "Memory Configuration",
          parameters: [
            {
              config_value: "128MB",
              format: "Bytes",
              name: "shared_buffers",
            },
            {
              config_value: "512MB",
              format: "Bytes",
              name: "effective_cache_size",
            },
            { config_value: "4MB", format: "Bytes", name: "work_mem" },
            {
              config_value: "128MB",
              format: "Bytes",
              name: "maintenance_work_mem",
            },
          ],
        },
        {
          category: "checkpoint_related",
          description: "Checkpoint Related Configuration",
          parameters: [
            {
              config_value: "2GB",
              format: "Bytes",
              name: "min_wal_size",
            },
            {
              config_value: "1GB",
              format: "Bytes",
              name: "max_wal_size",
            },
            {
              config_value: 0.5,
              format: "Float",
              name: "checkpoint_completion_target",
            },
            { config_value: "4MB", format: "Bytes", name: "wal_buffers" },
          ],
        },
        {
          category: "network_related",
          description: "Network Related Configuration",
          parameters: [
            {
              config_value: "*",
              format: "String",
              name: "listen_addresses",
            },
            {
              config_value: 100,
              format: "Decimal",
              name: "max_connections",
            },
          ],
        },
        {
          category: "hard_drive_type",
          description: "Hard Drive Configuration",
          parameters: [
            {
              config_value: 1.1,
              format: "Float",
              name: "random_page_cost",
            },
            {
              config_value: 200,
              format: "Decimal",
              name: "effective_io_concurrency",
            },
          ],
        },
        {
          category: "worker_processes",
          description: "Worker Processes",
          parameters: [
            {
              config_value: 1,
              format: "Decimal",
              name: "max_worker_processes",
            },
            {
              config_value: 2,
              format: "Decimal",
              name: "max_parallel_workers_per_gather",
            },
            {
              config_value: 1,
              format: "Decimal",
              name: "max_parallel_workers",
            },
          ],
        },
      ],
      environment: "Desktop",
    },
  ],
};

it("format structure", () => {
  assert.deepStrictEqual(structure(response.data), [
    {
      "category": "memory_related",
      "name": "Memory Configuration",
      "params": [],
    },
    {
      "category": "checkpoint_related",
      "name": "Checkpoint Related Configuration",
      "params": [],
    },
    {
      "category": "network_related",
      "name": "Network Related Configuration",
      "params": [],
    },
    {
      "category": "hard_drive_type",
      "name": "Hard Drive Configuration",
      "params": [],
    },
    {
      "category": "worker_processes",
      "name": "Worker Processes",
      "params": [],
    },
  ]);
});

it("should format configs", () => {
  assert.deepStrictEqual(formatConfigs(response.data), [
    {
      "category": "memory_related",
      "name": "Memory Configuration",
      "params": [
        {
          "desktop": "128MB",
          "documentation": undefined,
          "dw": "512MB",
          "mixed": "512MB",
          "name": "shared_buffers",
          "format": "Bytes",
          "oltp": "512MB",
          "web": "512MB",
        },
        {
          "desktop": "512MB",
          "documentation": undefined,
          "dw": "2GB",
          "mixed": "2GB",
          "name": "effective_cache_size",
          "format": "Bytes",
          "oltp": "2GB",
          "web": "2GB",
        },
        {
          "desktop": "4MB",
          "documentation": undefined,
          "dw": "10MB",
          "mixed": "10MB",
          "name": "work_mem",
          "format": "Bytes",
          "oltp": "20MB",
          "web": "20MB",
        },
        {
          "desktop": "128MB",
          "documentation": undefined,
          "dw": "256MB",
          "mixed": "128MB",
          "name": "maintenance_work_mem",
          "format": "Bytes",
          "oltp": "128MB",
          "web": "128MB",
        },
      ],
    },
    {
      "category": "checkpoint_related",
      "name": "Checkpoint Related Configuration",
      "params": [
        {
          "desktop": "2GB",
          "documentation": undefined,
          "dw": "2GB",
          "mixed": "512MB",
          "name": "min_wal_size",
          "format": "Bytes",
          "oltp": "1GB",
          "web": "512MB",
        },
        {
          "desktop": "1GB",
          "documentation": undefined,
          "dw": "6GB",
          "mixed": "2GB",
          "name": "max_wal_size",
          "format": "Bytes",
          "oltp": "3GB",
          "web": "2GB",
        },
        {
          "desktop": 0.5,
          "documentation": undefined,
          "dw": 0.9,
          "mixed": 0.9,
          "name": "checkpoint_completion_target",
          "format": "Float",
          "oltp": 0.9,
          "web": 0.7,
        },
        {
          "desktop": "4MB",
          "documentation": undefined,
          "dw": "15MB",
          "mixed": "15MB",
          "name": "wal_buffers",
          "format": "Bytes",
          "oltp": "15MB",
          "web": "15MB",
        },
      ],
    },
    {
      "category": "network_related",
      "name": "Network Related Configuration",
      "params": [
        {
          "desktop": "*",
          "documentation": undefined,
          "dw": "*",
          "mixed": "*",
          "name": "listen_addresses",
          "format": "String",
          "oltp": "*",
          "web": "*",
        },
        {
          "desktop": 100,
          "documentation": undefined,
          "dw": 100,
          "mixed": 100,
          "name": "max_connections",
          "format": "Decimal",
          "oltp": 100,
          "web": 100,
        },
      ],
    },
    {
      "category": "hard_drive_type",
      "name": "Hard Drive Configuration",
      "params": [
        {
          "desktop": 1.1,
          "documentation": undefined,
          "dw": 1.1,
          "mixed": 1.1,
          "name": "random_page_cost",
          "format": "Float",
          "oltp": 1.1,
          "web": 1.1,
        },
        {
          "desktop": 200,
          "documentation": undefined,
          "dw": 200,
          "mixed": 200,
          "name": "effective_io_concurrency",
          "format": "Decimal",
          "oltp": 200,
          "web": 200,
        },
      ],
    },
    {
      "category": "worker_processes",
      "name": "Worker Processes",
      "params": [
        {
          "desktop": 1,
          "documentation": undefined,
          "dw": 1,
          "mixed": 1,
          "name": "max_worker_processes",
          "format": "Decimal",
          "oltp": 1,
          "web": 1,
        },
        {
          "desktop": 2,
          "documentation": undefined,
          "dw": 2,
          "mixed": 2,
          "name": "max_parallel_workers_per_gather",
          "format": "Decimal",
          "oltp": 2,
          "web": 2,
        },
        {
          "desktop": 1,
          "documentation": undefined,
          "dw": 1,
          "mixed": 1,
          "name": "max_parallel_workers",
          "format": "Decimal",
          "oltp": 1,
          "web": 1,
        },
      ],
    },
  ]);
});
