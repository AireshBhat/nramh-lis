'use client';

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Switch } from '@/components/ui/switch';
import { 
  Settings, 
  Database, 
  Upload, 
  Monitor,
  Bell,
  Shield,
  Save
} from 'lucide-react';

export default function SettingsPage() {
  return (
    <div className="p-4 md:p-6 space-y-6">
      <div className="space-y-2">
        <h1 className="text-3xl font-bold">System Settings</h1>
        <p className="text-muted-foreground">
          Configure system preferences and settings
        </p>
      </div>

      <div className="grid gap-6 md:grid-cols-2">
        {/* Database Settings */}
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center space-x-2">
              <Database className="h-5 w-5" />
              <span>Database Configuration</span>
            </CardTitle>
            <CardDescription>
              Manage local database settings
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="space-y-2">
              <Label htmlFor="db-path">Database Path</Label>
              <Input 
                id="db-path" 
                placeholder="/path/to/database.db"
                defaultValue="/var/lib/lis/database.db"
              />
            </div>
            <div className="space-y-2">
              <Label htmlFor="backup-interval">Backup Interval (hours)</Label>
              <Input 
                id="backup-interval" 
                type="number"
                placeholder="24"
                defaultValue="24"
              />
            </div>
            <div className="flex items-center space-x-2">
              <Switch id="auto-backup" defaultChecked />
              <Label htmlFor="auto-backup">Enable automatic backups</Label>
            </div>
          </CardContent>
        </Card>

        {/* Upload Settings */}
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center space-x-2">
              <Upload className="h-5 w-5" />
              <span>HIS Upload Configuration</span>
            </CardTitle>
            <CardDescription>
              Configure Hospital Information System upload settings
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="space-y-2">
              <Label htmlFor="his-endpoint">HIS Endpoint URL</Label>
              <Input 
                id="his-endpoint" 
                placeholder="https://his.hospital.com/api"
                defaultValue="https://his.hospital.com/api"
              />
            </div>
            <div className="space-y-2">
              <Label htmlFor="retry-attempts">Max Retry Attempts</Label>
              <Input 
                id="retry-attempts" 
                type="number"
                placeholder="3"
                defaultValue="3"
              />
            </div>
            <div className="space-y-2">
              <Label htmlFor="upload-timeout">Upload Timeout (seconds)</Label>
              <Input 
                id="upload-timeout" 
                type="number"
                placeholder="30"
                defaultValue="30"
              />
            </div>
            <div className="flex items-center space-x-2">
              <Switch id="auto-upload" defaultChecked />
              <Label htmlFor="auto-upload">Enable automatic uploads</Label>
            </div>
          </CardContent>
        </Card>

        {/* System Monitoring */}
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center space-x-2">
              <Monitor className="h-5 w-5" />
              <span>System Monitoring</span>
            </CardTitle>
            <CardDescription>
              Configure system monitoring and alerts
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="space-y-2">
              <Label htmlFor="log-level">Log Level</Label>
              <select 
                id="log-level" 
                className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
                defaultValue="INFO"
              >
                <option value="ERROR">ERROR</option>
                <option value="WARN">WARN</option>
                <option value="INFO">INFO</option>
                <option value="DEBUG">DEBUG</option>
              </select>
            </div>
            <div className="space-y-2">
              <Label htmlFor="log-retention">Log Retention (days)</Label>
              <Input 
                id="log-retention" 
                type="number"
                placeholder="30"
                defaultValue="30"
              />
            </div>
            <div className="flex items-center space-x-2">
              <Switch id="health-checks" defaultChecked />
              <Label htmlFor="health-checks">Enable health checks</Label>
            </div>
          </CardContent>
        </Card>

        {/* Notifications */}
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center space-x-2">
              <Bell className="h-5 w-5" />
              <span>Notification Settings</span>
            </CardTitle>
            <CardDescription>
              Configure system notifications and alerts
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="space-y-2">
              <Label htmlFor="email-notifications">Email for Notifications</Label>
              <Input 
                id="email-notifications" 
                type="email"
                placeholder="admin@hospital.com"
                defaultValue="admin@hospital.com"
              />
            </div>
            <div className="space-y-3">
              <div className="flex items-center space-x-2">
                <Switch id="error-alerts" defaultChecked />
                <Label htmlFor="error-alerts">Error alerts</Label>
              </div>
              <div className="flex items-center space-x-2">
                <Switch id="upload-failures" defaultChecked />
                <Label htmlFor="upload-failures">Upload failure alerts</Label>
              </div>
              <div className="flex items-center space-x-2">
                <Switch id="system-status" />
                <Label htmlFor="system-status">System status updates</Label>
              </div>
            </div>
          </CardContent>
        </Card>

        {/* Security Settings */}
        <Card className="md:col-span-2">
          <CardHeader>
            <CardTitle className="flex items-center space-x-2">
              <Shield className="h-5 w-5" />
              <span>Security Settings</span>
            </CardTitle>
            <CardDescription>
              Configure security and access control settings
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="grid gap-4 md:grid-cols-2">
              <div className="space-y-2">
                <Label htmlFor="api-key">API Key</Label>
                <Input 
                  id="api-key" 
                  type="password"
                  placeholder="Enter API key"
                  defaultValue="••••••••••••••••"
                />
              </div>
              <div className="space-y-2">
                <Label htmlFor="session-timeout">Session Timeout (minutes)</Label>
                <Input 
                  id="session-timeout" 
                  type="number"
                  placeholder="60"
                  defaultValue="60"
                />
              </div>
            </div>
            <div className="space-y-3">
              <div className="flex items-center space-x-2">
                <Switch id="audit-logs" defaultChecked />
                <Label htmlFor="audit-logs">Enable audit logging</Label>
              </div>
              <div className="flex items-center space-x-2">
                <Switch id="encryption" defaultChecked />
                <Label htmlFor="encryption">Enable data encryption</Label>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>

      <div className="flex justify-end space-x-2">
        <Button variant="outline">
          Reset to Defaults
        </Button>
        <Button>
          <Save className="h-4 w-4 mr-2" />
          Save Settings
        </Button>
      </div>
    </div>
  );
}